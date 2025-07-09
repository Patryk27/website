use super::{Element, Printer, Result, Spanned};

impl Printer<'_> {
    pub(super) fn add_shdr(&mut self, el: Element) -> Result<()> {
        el.assert_no_attrs()?;

        self.add_any(Element {
            name: Spanned::dummy("h3".into()),
            attrs: Vec::new(),
            children: el.children,
        })
    }
}
