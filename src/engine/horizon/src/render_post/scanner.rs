use super::{Attr, Elem, Error, Node, Result, Span, Spanned};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone, Debug)]
pub struct Scanner<'a> {
    src: Peekable<Chars<'a>>,
    pos: usize,
    context: Vec<(usize, String)>,
}

impl<'a> Scanner<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src: src.chars().peekable(),
            pos: 0,
            context: Default::default(),
        }
    }

    fn char(&mut self) -> Result<char> {
        self.char_opt().ok_or_else(|| {
            Error::new("unexpected end of file", Span::char(self.pos - 1))
        })
    }

    fn char_opt(&mut self) -> Option<char> {
        self.pos += 1;
        self.src.next()
    }

    fn peek_char_opt(&mut self) -> Option<char> {
        self.src.peek().copied()
    }

    fn eat(&mut self, expected: char) -> Result<()> {
        let actual = self.char()?;

        if actual == expected {
            Ok(())
        } else {
            Err(Error::new(
                format!("expected `{expected}`"),
                Span::char(self.pos - 1),
            ))
        }
    }

    fn try_eating<const N: usize>(&mut self, expected: [char; N]) -> bool {
        if let Some(actual) = self.peek_char_opt()
            && expected.contains(&actual)
        {
            _ = self.char();
            return true;
        }

        false
    }

    fn whitespace(&mut self) {
        while self.try_eating([' ', '\n']) {
            //
        }
    }

    fn text(&mut self) -> Result<Spanned<String>> {
        self.context
            .push((self.pos, "... when reading this text".into()));

        let mut out = Spanned::new(String::new(), Span::char(self.pos));

        out.push(self.char()?);

        while let Some(ch) = self.peek_char_opt() {
            match ch {
                '<' => break,
                _ => out.push(self.char()?),
            }
        }

        out.span_mut().end = self.pos;

        self.context.pop();

        Ok(out)
    }

    fn comment(&mut self) -> Result<Spanned<String>> {
        self.context
            .push((self.pos, "... when reading this comment".into()));

        self.eat('-')?;
        self.eat('-')?;

        let mut out = Spanned::new(String::new(), Span::char(self.pos));

        loop {
            out.push(self.char()?);

            if out.ends_with("-->") {
                let len = out.len();

                out.drain(len - 3..).for_each(drop);
                break;
            }
        }

        out.span_mut().end = self.pos - 2;

        self.context.pop();

        Ok(out)
    }

    fn elem(&mut self) -> Result<Spanned<Elem>> {
        self.context
            .push((self.pos - 1, "... when reading this element".into()));

        let mut out = Spanned::new(Elem::default(), Span::char(self.pos - 1));

        if self.elem_tag(&mut out)? {
            self.elem_children(&mut out)?;
        }

        out.span_mut().end = self.pos;

        self.context.pop();

        Ok(out)
    }

    fn elem_tag(&mut self, out: &mut Spanned<Elem>) -> Result<bool> {
        out.name.span_mut().start = self.pos;

        let expect_attrs = self.elem_tag_name(out)?;

        out.name.span_mut().end = self.pos - 1;

        let expect_children = if expect_attrs {
            self.elem_tag_attrs(out)?
        } else {
            true
        };

        Ok(expect_children)
    }

    fn elem_tag_name(&mut self, out: &mut Spanned<Elem>) -> Result<bool> {
        loop {
            match self.char()? {
                '>' => {
                    return Ok(false);
                }

                ' ' | '\n' => {
                    return Ok(true);
                }

                ch @ ('a'..='z' | '0'..='9' | '-') => {
                    out.name.push(ch);
                }

                _ => {
                    return Err(Error::new(
                        "unexpected character",
                        Span::char(self.pos - 1),
                    ));
                }
            }
        }
    }

    fn elem_tag_attrs(&mut self, out: &mut Spanned<Elem>) -> Result<bool> {
        loop {
            self.whitespace();

            let mut name = Spanned::new(String::new(), Span::char(self.pos));

            let expect = loop {
                match self.char()? {
                    '=' => break "attr-value",
                    ' ' => break "next-attr",
                    '>' => break "header-end,children",
                    '/' => break "header-end,no-children",

                    ch @ ('a'..='z' | '-') => name.push(ch),

                    _ => {
                        return Err(Error::new(
                            "unexpected character",
                            Span::char(self.pos - 1),
                        ));
                    }
                }
            };

            name.span_mut().end = self.pos - 1;

            if !name.is_empty() {
                if expect == "attr-value" {
                    let mut value =
                        Spanned::new(String::new(), Span::char(self.pos));

                    self.eat('"')?;

                    loop {
                        match self.char()? {
                            '"' => break,
                            ch => value.push(ch),
                        }
                    }

                    value.span_mut().end = self.pos;

                    out.attrs.push(Attr {
                        name,
                        value: Some(value),
                    });
                } else {
                    out.attrs.push(Attr { name, value: None });
                }
            }

            match expect {
                "attr-value" | "next-attr" => {
                    //
                }

                "header-end,children" => {
                    return Ok(true);
                }

                "header-end,no-children" => {
                    self.eat('>')?;
                    return Ok(false);
                }

                _ => unreachable!(),
            }
        }
    }

    fn elem_children(&mut self, out: &mut Spanned<Elem>) -> Result<()> {
        loop {
            match self.node()? {
                NodeOrClosingTag::Node(node) => {
                    out.children.push(node);
                }

                NodeOrClosingTag::ClosingTag(tag) => {
                    if tag == out.name {
                        break;
                    } else {
                        return Err(Error::new(
                            format!(
                                "mismatched closing tag, expected `</{}>`",
                                *out.name,
                            ),
                            tag.span(),
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    fn closing_tag(&mut self) -> Result<Spanned<String>> {
        self.context
            .push((self.pos - 2, "... when reading this closing tag".into()));

        let mut out = Spanned::new(String::new(), Span::char(self.pos - 2));

        loop {
            match self.char()? {
                '>' => {
                    break;
                }

                ch @ ('a'..='z' | '0'..='9' | '-') => {
                    out.push(ch);
                }

                _ => {
                    return Err(Error::new(
                        "unexpected character",
                        Span::char(self.pos - 1),
                    ));
                }
            }
        }

        out.span_mut().end = self.pos;

        self.context.pop();

        Ok(out)
    }

    fn node(&mut self) -> Result<NodeOrClosingTag> {
        if self.try_eating(['<']) {
            if self.try_eating(['!']) {
                Ok(NodeOrClosingTag::Node(Node::Comment(self.comment()?)))
            } else if self.try_eating(['/']) {
                Ok(NodeOrClosingTag::ClosingTag(self.closing_tag()?))
            } else {
                Ok(NodeOrClosingTag::Node(Node::Elem(self.elem()?)))
            }
        } else {
            Ok(NodeOrClosingTag::Node(Node::Text(self.text()?)))
        }
    }
}

impl Iterator for Scanner<'_> {
    type Item = Result<Node>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.src.peek().is_some() {
            match self.node() {
                Ok(NodeOrClosingTag::Node(node)) => Some(Ok(node)),
                Ok(NodeOrClosingTag::ClosingTag(node)) => {
                    Some(Err(Error::new("unexpected closing tag", node.span())))
                }

                Err(mut err) => {
                    for (span, label) in &self.context {
                        err = err.with_label(label, Span::char(*span));
                    }

                    Some(Err(err))
                }
            }
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
enum NodeOrClosingTag {
    Node(Node),
    ClosingTag(Spanned<String>),
}
