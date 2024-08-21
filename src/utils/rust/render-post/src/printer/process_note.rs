use super::{Element, MessageResult, Printer};

impl Printer<'_> {
    pub(super) fn process_note(&mut self, el: Element) -> MessageResult<()> {
        el.check_no_more_attrs()?;

        _ = write!(self.out, r#"<aside class="note">"#);

        for el in el.children {
            self.process(el)?;
        }

        _ = write!(self.out, r#"</aside>"#);

        Ok(())
    }
}
