use super::{Element, Message, MessageResult, Node, Printer};
use itertools::Itertools;

impl Printer<'_> {
    pub(super) fn process_ref(&mut self, mut el: Element) -> MessageResult<()> {
        let (_, id) = el.remove_attr("id").ok_or_else(|| {
            Message::new("missing attribute: `id`", el.name.span())
        })?;

        if let Some((attr, _)) = el.attrs.first() {
            return Err(Message::new("unknown attribute", attr.span()));
        }

        let url = el
            .children
            .into_iter()
            .flat_map(|chd| {
                if let Node::Text(chd) = chd {
                    Some(chd.into_inner())
                } else {
                    None
                }
            })
            .join("")
            .trim()
            .to_owned();

        let span = id.span();

        if let Some((_, prev_span)) =
            self.refs.insert(id.into_inner(), (url, span))
        {
            return Err(Message::new("ref overwrites another ref", span)
                .with_label("... defined here", prev_span));
        }

        Ok(())
    }
}
