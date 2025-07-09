use super::{Attr, Element, Node, Printer, Result, Spanned};
use std::iter;

impl Printer<'_> {
    pub(super) fn add_toc(&mut self, el: Element) -> Result<()> {
        el.assert_no_attrs()?;
        el.assert_no_children()?;

        let children = self
            .headers
            .drain(..)
            .map(|header| {
                let header = Element {
                    name: Spanned::dummy("a".into()),
                    attrs: vec![Attr {
                        name: Spanned::dummy("href".into()),
                        value: Some(Spanned::dummy(format!("#{}", header.id))),
                    }],
                    children: header.name,
                };

                let header = Element {
                    name: Spanned::dummy("li".into()),
                    attrs: vec![],
                    children: iter::once(Node::Text(Spanned::dummy(
                        "&gt; ".into(),
                    )))
                    .chain(iter::once(Node::Element(Spanned::dummy(header))))
                    .collect(),
                };

                Node::Element(Spanned::dummy(header))
            })
            .collect();

        self.add_any(Element {
            name: Spanned::dummy("ul".into()),
            attrs: vec![Attr {
                name: Spanned::dummy("class".into()),
                value: Some(Spanned::dummy("toc".into())),
            }],
            children,
        })
    }
}
