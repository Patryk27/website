use super::{Message, MessageResult, Span, Spanned};

#[derive(Clone, Debug, Default)]
pub struct Element {
    pub name: Spanned<String>,
    pub attrs: Vec<(Spanned<String>, Spanned<String>)>,
    pub children: Vec<Node>,
}

impl Element {
    pub fn attr(
        &self,
        name: &str,
    ) -> Option<&(Spanned<String>, Spanned<String>)> {
        self.attrs.iter().find(|(attr_name, _)| **attr_name == name)
    }

    pub fn remove_attr(
        &mut self,
        name: &str,
    ) -> Option<(Spanned<String>, Spanned<String>)> {
        self.attrs
            .extract_if(|(attr_name, _)| **attr_name == name)
            .next()
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
