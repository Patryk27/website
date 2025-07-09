use super::{Error, Result, Span, Spanned};

#[derive(Clone, Debug)]
pub enum Node {
    Text(Spanned<String>),
    Comment(Spanned<String>),
    Element(Spanned<Element>),
}

impl Node {
    pub fn span(&self) -> Span {
        match self {
            Node::Text(node) => node.span(),
            Node::Comment(node) => node.span(),
            Node::Element(node) => node.span(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Element {
    pub name: Spanned<String>,
    pub attrs: Vec<Attr>,
    pub children: Vec<Node>,
}

impl Element {
    pub fn attr(&self, name: &str) -> Result<&str> {
        self.attrs
            .iter()
            .find_map(|attr| {
                if *attr.name == name {
                    attr.value.as_ref().map(|value| value.as_str())
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                Error::new(
                    format!("missing attribute: `{name}`"),
                    self.name.span(),
                )
            })
    }

    pub fn remove_attr(&mut self, name: &str) -> Result<Attr> {
        self.attrs
            .extract_if(.., |attr| *attr.name == name)
            .next()
            .ok_or_else(|| {
                Error::new(
                    format!("missing attribute: `{name}`"),
                    self.name.span(),
                )
            })
    }

    pub fn assert_no_attrs(&self) -> Result<()> {
        if let Some(attr) = self.attrs.first() {
            Err(Error::new("unexpected attribute", attr.name.span()))
        } else {
            Ok(())
        }
    }

    pub fn assert_no_children(&self) -> Result<()> {
        if let Some(child) = self.children.first() {
            Err(Error::new("unexpected child", child.span()))
        } else {
            Ok(())
        }
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
