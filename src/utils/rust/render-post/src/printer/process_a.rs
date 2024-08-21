use super::{Attr, Element, Message, MessageResult, Node, Printer, Spanned};

impl Printer<'_> {
    pub(super) fn process_a(&mut self, mut el: Element) -> MessageResult<()> {
        if let Some(ref_attr) = el.remove_attr_opt("ref") {
            let url = if let Some(ref_val) = ref_attr.value {
                self.refs
                    .remove(&Some(ref_val.to_string()))
                    .ok_or_else(|| {
                        Message::new(
                            format!("unknown ref: `{}`", ref_val.as_str()),
                            ref_val.span(),
                        )
                    })?
                    .0
            } else {
                self.refs
                    .remove(&None)
                    .ok_or_else(|| {
                        Message::new("unknown ref", ref_attr.name.span())
                    })?
                    .0
            };

            el.attrs.push(Attr {
                name: Spanned::dummy("href".into()),
                value: Some(Spanned::dummy(url)),
            });
        }

        let href = el.attr("href")?;

        if el.children.is_empty() {
            el.children.push(Node::Text(href.value()?.clone()));
        }

        self.process_generic(el)
    }
}
