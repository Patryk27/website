use super::{Elem, Printer, Result, Spanned};

impl Printer<'_> {
    pub(super) fn add_shdr(&mut self, el: Elem) -> Result<()> {
        el.assert_no_attrs()?;

        self.add_any(Elem {
            name: Spanned::dummy("h3".into()),
            attrs: Vec::new(),
            children: el.children,
        })
    }
}
