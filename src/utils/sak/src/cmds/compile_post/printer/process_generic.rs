use super::{Element, MessageResult, Printer};

impl Printer<'_> {
    pub(super) fn process_generic(&mut self, el: Element) -> MessageResult<()> {
        _ = write!(self.out, "<{}", *el.name);

        for (attr_name, attr_value) in &el.attrs {
            _ = write!(self.out, r#" {}="{}""#, **attr_name, **attr_value);
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
