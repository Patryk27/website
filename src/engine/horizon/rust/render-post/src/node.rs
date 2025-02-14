use crate::{Error, Result, Span, Spanned};

#[derive(Clone, Debug)]
pub enum Node {
    Text(Spanned<String>),
    Comment(Spanned<String>),
    Elem(Spanned<Elem>),
}

impl Node {
    pub fn span(&self) -> Span {
        match self {
            Node::Text(el) => el.span(),
            Node::Comment(el) => el.span(),
            Node::Elem(el) => el.span(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Elem {
    pub name: Spanned<String>,
    pub attrs: Vec<Attr>,
    pub children: Vec<Node>,
}

impl Elem {
    pub fn remove_attr(&mut self, key: &str) -> Result<Attr> {
        self.remove_attr_opt(key).ok_or_else(|| {
            Error::new(
                format!("missing attribute: `{}`", key),
                self.name.span(),
            )
        })
    }

    pub fn remove_attr_opt(&mut self, key: &str) -> Option<Attr> {
        self.attrs.extract_if(.., |attr| *attr.name == key).next()
    }

    pub fn assert_no_attrs(&self) -> Result<()> {
        if let Some(attr) = self.attrs.first() {
            Err(Error::new("unknown attribute", attr.name.span()))
        } else {
            Ok(())
        }
    }

    pub fn comment(&self) -> Result<&Spanned<String>> {
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
                    return Err(Error::new("unexpected node", el.span()));
                }
            }
        }

        found.ok_or_else(|| Error::new("expected a comment", self.name.span()))
    }
}

#[derive(Clone, Debug)]
pub struct Attr {
    pub name: Spanned<String>,
    pub value: Option<Spanned<String>>,
}

impl Attr {
    pub fn value(&self) -> Result<&Spanned<String>> {
        self.value.as_ref().ok_or_else(|| {
            Error::new(
                format!("attribute `{}` cannot be empty", self.name.as_str()),
                self.name.span(),
            )
        })
    }

    pub fn into_value(self) -> Result<Spanned<String>> {
        self.value.ok_or_else(|| {
            Error::new(
                format!("attribute `{}` cannot be empty", self.name.as_str()),
                self.name.span(),
            )
        })
    }
}
