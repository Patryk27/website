use super::{Element, Printer, Result};

impl Printer<'_> {
    pub(super) fn add_code_title(&mut self, el: Element) -> Result<()> {
        el.assert_no_attrs()?;

        _ = write!(self.out, r#"<div class="code-title">"#);

        for el in el.children {
            self.add(el)?;
        }

        _ = write!(self.out, r#"</div>"#);

        Ok(())
    }
}
