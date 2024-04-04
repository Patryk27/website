use super::{Element, MessageResult, Printer};

impl Printer<'_> {
    pub(super) fn process_listing_title(
        &mut self,
        el: Element,
    ) -> MessageResult<()> {
        el.check_no_more_attrs()?;

        _ = write!(self.out, r#"<div class="listing-title">"#);

        for el in el.children {
            self.process(el)?;
        }

        _ = write!(self.out, r#"</div>"#);

        Ok(())
    }
}
