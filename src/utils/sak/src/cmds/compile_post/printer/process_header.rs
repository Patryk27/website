use super::{Attr, Element, MessageResult, Node, Printer, Spanned};
use std::mem;

impl Printer<'_> {
    pub(super) fn process_header(
        &mut self,
        mut el: Element,
    ) -> MessageResult<()> {
        let id = el.remove_attr("id")?;

        el.check_no_more_attrs()?;

        self.process_generic(Element {
            name: Spanned::dummy("h2".into()),
            attrs: Default::default(),
            children: vec![Node::Element(Spanned::dummy(Element {
                name: Spanned::dummy("a".into()),
                attrs: vec![Attr {
                    name: Spanned::dummy("href".into()),
                    value: Some(Spanned::dummy(format!(
                        "#{}",
                        id.value()?.as_str()
                    ))),
                }],
                children: mem::take(&mut el.children),
            }))],
        })
    }
}
