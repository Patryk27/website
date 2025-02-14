use super::Printer;
use crate::{Elem, Result};

impl Printer<'_> {
    pub(super) fn add_note(&mut self, el: Elem) -> Result<()> {
        el.assert_no_attrs()?;

        _ = write!(self.out, r#"<aside class="note">"#);

        for el in el.children {
            self.add(el)?;
        }

        _ = write!(self.out, r#"</aside>"#);

        Ok(())
    }
}
