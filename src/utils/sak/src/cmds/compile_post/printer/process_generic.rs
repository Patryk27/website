use super::{Element, MessageResult, Printer};

impl Printer<'_> {
    pub(super) fn process_generic(&mut self, el: Element) -> MessageResult<()> {
        _ = write!(self.out, "<{}", *el.name);

        for attr in &el.attrs {
            if let Some(attr_val) = &attr.value {
                _ = write!(self.out, r#" {}="{}""#, *attr.name, **attr_val);
            } else {
                _ = write!(self.out, r#" {}"#, *attr.name);
            }
        }

        if el.children.is_empty() {
            _ = write!(self.out, " />");
        } else {
            _ = write!(self.out, ">");

            for el in el.children {
                self.process(el)?;
            }

            _ = write!(self.out, "</{}>", *el.name);
        }

        Ok(())
    }
}
