use super::{Element, Message, MessageResult, Node, Printer, Spanned};

impl Printer<'_> {
    pub(super) fn process_a(&mut self, mut el: Element) -> MessageResult<()> {
        if let Some((_, attr)) = el.remove_attr("ref") {
            let (url, _) =
                self.refs.remove(attr.as_str()).ok_or_else(|| {
                    Message::new(
                        format!("unknown ref: `{}`", attr.as_str()),
                        attr.span(),
                    )
                })?;

            el.attrs
                .push((Spanned::dummy("href".into()), Spanned::dummy(url)));
        }

        let (_, href) = el.attr("href").ok_or_else(|| {
            Message::new("missing attribute: `href`", el.name.span())
        })?;

        if el.children.is_empty() {
            el.children.push(Node::Text(href.clone()));
        }

        self.process_generic(el)
    }
}
