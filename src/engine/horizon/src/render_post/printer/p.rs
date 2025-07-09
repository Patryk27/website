use super::{Element, Error, Node, Printer, Result};

impl Printer<'_> {
    pub(super) fn add_p(&mut self, el: Element) -> Result<()> {
        if let Some(Node::Text(text)) = el.children.first()
            && !text.starts_with('\n')
        {
            return Err(Error::new(
                "p's body should start on the next line",
                text.span(),
            ));
        }

        self.add_any(el)
    }
}
