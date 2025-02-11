use super::{Message, MessageResult, Span, Spanned};

#[derive(Clone, Debug, Default)]
pub struct Element {
    pub name: Spanned<String>,
    pub attrs: Vec<Attr>,
    pub children: Vec<Node>,
}

impl Element {
    pub fn attr_opt(&self, key: &str) -> Option<&Attr> {
        self.attrs.iter().find(|attr| *attr.name == key)
    }

    pub fn attr(&self, key: &str) -> MessageResult<&Attr> {
        self.attr_opt(key).ok_or_else(|| {
            Message::new(
                format!("missing attribute: `{}`", key),
                self.name.span(),
            )
        })
    }

    pub fn remove_attr_opt(&mut self, key: &str) -> Option<Attr> {
        self.attrs.extract_if(.., |attr| *attr.name == key).next()
    }

    pub fn remove_attr(&mut self, key: &str) -> MessageResult<Attr> {
        self.remove_attr_opt(key).ok_or_else(|| {
            Message::new(
                format!("missing attribute: `{}`", key),
                self.name.span(),
            )
        })
    }

    pub fn check_no_more_attrs(&self) -> MessageResult<()> {
        if let Some(attr) = self.attrs.first() {
            Err(Message::new("unknown attribute", attr.name.span()))
        } else {
            Ok(())
        }
    }

    pub fn comment(&self) -> MessageResult<&Spanned<String>> {
        let mut found = None;

        for el in &self.children {
            match el {
                Node::Text(text) if text.trim().is_empty() => {
                    //
                }

                Node::Comment(comment) if found.is_none() => {
                    found = Some(comment);
                }

                _ => {
                    return Err(Message::new("unexpected node", el.span()));
                }
            }
        }

        found
            .ok_or_else(|| Message::new("expected a comment", self.name.span()))
    }
}

#[derive(Clone, Debug)]
pub struct Attr {
    pub name: Spanned<String>,
    pub value: Option<Spanned<String>>,
}

impl Attr {
    pub fn value(&self) -> MessageResult<&Spanned<String>> {
        self.value.as_ref().ok_or_else(|| {
            Message::new("attribute `{}` cannot be empty", self.name.span())
        })
    }
}

#[derive(Clone, Debug)]
pub enum Node {
    Text(Spanned<String>),
    Comment(Spanned<String>),
    Element(Spanned<Element>),
}

impl Node {
    pub fn span(&self) -> Span {
        match self {
            Node::Text(el) => el.span(),
            Node::Comment(el) => el.span(),
            Node::Element(el) => el.span(),
        }
    }
}
