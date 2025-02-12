use super::Printer;
use crate::{Attr, Elem, Node, Result, Spanned};

impl Printer<'_> {
    pub(super) fn add_h2(&mut self, mut el: Elem) -> Result<()> {
        let id = el.remove_attr("id")?;
        let id = id.value()?;

        el.assert_no_attrs()?;

        self.add_ex(Elem {
            name: Spanned::dummy("h2".into()),
            attrs: vec![Attr {
                name: Spanned::dummy("id".into()),
                value: Some(id.clone()),
            }],
            children: vec![Node::Elem(Spanned::dummy(Elem {
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
