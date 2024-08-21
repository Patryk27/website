use super::{Element, Message, MessageResult, Node, Printer};
use itertools::Itertools;

impl Printer<'_> {
    pub(super) fn process_ref(&mut self, mut el: Element) -> MessageResult<()> {
        let id_and_span = el.remove_attr_opt("id").and_then(|attr| attr.value);

        el.check_no_more_attrs()?;

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

        let span;
        let id;

        if let Some(id_and_span) = id_and_span {
            span = id_and_span.span();
            id = Some(id_and_span.into_inner());
        } else {
            span = el.name.span();
            id = None;
        }

        if let Some((_, prev_span)) = self.refs.insert(id, (url, span)) {
            return Err(Message::new("ref overwrites another ref", span)
                .with_label("ref defined here before", prev_span));
        }

        Ok(())
    }
}
