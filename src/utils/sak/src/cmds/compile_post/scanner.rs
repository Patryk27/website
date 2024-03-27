use super::{Element, Message, MessageResult, Node, Span, Spanned};
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

    fn char(&mut self) -> MessageResult<char> {
        self.char_opt().ok_or_else(|| {
            Message::new("unexpected end of file", Span::char(self.pos - 1))
        })
    }

    fn char_opt(&mut self) -> Option<char> {
        self.pos += 1;
        self.src.next()
    }

    fn peek_char_opt(&mut self) -> Option<char> {
        self.src.peek().copied()
    }

    fn eat(&mut self, expected: char) -> MessageResult<()> {
        let actual = self.char()?;

        if actual == expected {
            Ok(())
        } else {
            Err(Message::new(
                format!("expected `{}`", expected),
                Span::char(self.pos - 1),
            ))
        }
    }

    fn skip<const N: usize>(&mut self, expected: [char; N]) -> bool {
        if let Some(actual) = self.peek_char_opt() {
            if expected.contains(&actual) {
                _ = self.char();
                return true;
            }
        }

        false
    }

    fn whitespace(&mut self) {
        while self.skip([' ', '\n']) {
            //
        }
    }

    fn text(&mut self) -> MessageResult<Spanned<String>> {
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

        out.span_mut().end = self.pos - 1;

        self.context.pop();

        Ok(out)
    }

    fn comment(&mut self) -> MessageResult<Spanned<String>> {
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

        out.span_mut().end = self.pos - 3;

        self.context.pop();

        Ok(out)
    }

    fn element(&mut self) -> MessageResult<Spanned<Element>> {
        self.context
            .push((self.pos - 1, "... when reading this element".into()));

        let mut out =
            Spanned::new(Element::default(), Span::char(self.pos - 1));

        let mut expect_attrs = false;
        let mut expect_children = true;

        // ---

        out.name.span_mut().beg = self.pos;

        loop {
            match self.char()? {
                '>' => {
                    break;
                }

                ' ' | '\n' => {
                    expect_attrs = true;
                    break;
                }

                ch @ ('a'..='z' | '0'..='9' | '-') => {
                    out.name.push(ch);
                }

                _ => {
                    return Err(Message::new(
                        "unexpected character",
                        Span::char(self.pos - 1),
                    ))
                }
            }
        }

        out.name.span_mut().end = self.pos - 2;

        // ---

        if expect_attrs {
            loop {
                self.whitespace();

                if self.skip(['>']) {
                    break;
                }

                if self.skip(['/']) {
                    self.eat('>')?;
                    expect_children = false;
                    break;
                }

                // --

                let mut attr_name =
                    Spanned::new(String::new(), Span::char(self.pos));

                loop {
                    match self.char()? {
                        '=' => break,
                        ch @ ('a'..='z' | '-') => attr_name.push(ch),

                        _ => {
                            return Err(Message::new(
                                "unexpected character",
                                Span::char(self.pos - 1),
                            ))
                        }
                    }
                }

                attr_name.span_mut().end = self.pos - 2;

                // --

                let mut attr_value =
                    Spanned::new(String::new(), Span::char(self.pos));

                self.eat('"')?;

                loop {
                    match self.char()? {
                        '"' => break,
                        ch => attr_value.push(ch),
                    }
                }

                attr_value.span_mut().end = self.pos - 1;

                // --

                out.attrs.push((attr_name, attr_value));
            }
        }

        // ---

        if expect_children {
            loop {
                match self.node()? {
                    NodeOrClosingTag::Node(node) => {
                        out.children.push(node);
                    }

                    NodeOrClosingTag::ClosingTag(tag) => {
                        if tag == out.name {
                            break;
                        } else {
                            return Err(Message::new(
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
        }

        out.span_mut().end = self.pos - 1;

        self.context.pop();

        Ok(out)
    }

    fn closing_tag(&mut self) -> MessageResult<Spanned<String>> {
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
                    return Err(Message::new(
                        "unexpected character",
                        Span::char(self.pos - 1),
                    ))
                }
            }
        }

        out.span_mut().end = self.pos - 1;

        self.context.pop();

        Ok(out)
    }

    fn node(&mut self) -> MessageResult<NodeOrClosingTag> {
        if self.skip(['<']) {
            if self.skip(['!']) {
                Ok(NodeOrClosingTag::Node(Node::Comment(self.comment()?)))
            } else if self.skip(['/']) {
                Ok(NodeOrClosingTag::ClosingTag(self.closing_tag()?))
            } else {
                Ok(NodeOrClosingTag::Node(Node::Element(self.element()?)))
            }
        } else {
            Ok(NodeOrClosingTag::Node(Node::Text(self.text()?)))
        }
    }
}

impl Iterator for Scanner<'_> {
    type Item = MessageResult<Node>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.src.peek().is_some() {
            match self.node() {
                Ok(NodeOrClosingTag::Node(node)) => Some(Ok(node)),
                Ok(NodeOrClosingTag::ClosingTag(node)) => Some(Err(
                    Message::new("unexpected closing tag", node.span()),
                )),

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
