use super::Printer;
use crate::{Elem, Error, Node, Result};

impl Printer<'_> {
    pub(super) fn add_p(&mut self, el: Elem) -> Result<()> {
        if let Some(Node::Text(text)) = el.children.first() {
            if !text.starts_with('\n') {
                return Err(Error::new(
                    "p's body should start on the next line",
                    text.span(),
                ));
            }
        }

        self.add_ex(el)
    }
}
