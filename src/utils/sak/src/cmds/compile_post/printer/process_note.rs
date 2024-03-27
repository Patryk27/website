use super::{Element, Message, MessageResult, Printer};

impl Printer<'_> {
    pub(super) fn process_note(&mut self, el: Element) -> MessageResult<()> {
        if let Some((attr, _)) = el.attrs.first() {
            return Err(Message::new("unknown attribute", attr.span()));
        }

        _ = write!(self.out, r#"<aside class="note">"#);

        for el in el.children {
            self.process(el)?;
        }

        _ = write!(self.out, r#"</aside>"#);

        Ok(())
    }
}
