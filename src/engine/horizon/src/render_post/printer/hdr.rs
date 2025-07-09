use super::{Attr, Element, Header, Node, Printer, Result, Spanned};

impl Printer<'_> {
    pub(super) fn scan_hdr(&mut self, el: &Element) {
        let Ok(id) = el.attr("id") else {
            return;
        };

        self.headers.push(Header {
            id: id.into(),
            name: el.children.clone(),
        });
    }

    pub(super) fn add_hdr(&mut self, mut el: Element) -> Result<()> {
        let id = el.remove_attr("id")?;
        let id = id.value()?;

        el.assert_no_attrs()?;

        self.add_any(Element {
            name: Spanned::dummy("h2".into()),
            attrs: vec![Attr {
                name: Spanned::dummy("id".into()),
                value: Some(id.clone()),
            }],
            children: vec![Node::Element(Spanned::dummy(Element {
                name: Spanned::dummy("a".into()),
                attrs: vec![Attr {
                    name: Spanned::dummy("href".into()),
                    value: Some(Spanned::dummy(format!("#{}", id.as_str()))),
                }],
                children: el.children,
            }))],
        })
    }
}
