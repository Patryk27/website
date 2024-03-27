use super::{Element, Message, MessageResult, Node, Printer, Spanned};
use std::mem;

impl Printer<'_> {
    pub(super) fn process_header(
        &mut self,
        mut el: Element,
    ) -> MessageResult<()> {
        let (_, id) = el.remove_attr("id").ok_or_else(|| {
            Message::new("missing attribute: `id`", el.name.span())
        })?;

        if let Some((attr, _)) = el.attrs.first() {
            return Err(Message::new("unknown attribute", attr.span()));
        }

        self.process_generic(Element {
            name: Spanned::dummy("h2".into()),
            attrs: Default::default(),
            children: vec![Node::Element(Spanned::dummy(Element {
                name: Spanned::dummy("a".into()),
                attrs: vec![(
                    Spanned::dummy("href".into()),
                    Spanned::dummy(format!("#{}", id.as_str())),
                )],
                children: mem::take(&mut el.children),
            }))],
        })
    }
}
