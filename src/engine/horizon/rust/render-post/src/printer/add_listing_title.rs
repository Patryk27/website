use super::Printer;
use crate::{Elem, Result};

impl Printer<'_> {
    pub(super) fn add_listing_title(&mut self, el: Elem) -> Result<()> {
        el.assert_no_attrs()?;

        _ = write!(self.out, r#"<div class="listing-title">"#);

        for el in el.children {
            self.add(el)?;
        }

        _ = write!(self.out, r#"</div>"#);

        Ok(())
    }
}
