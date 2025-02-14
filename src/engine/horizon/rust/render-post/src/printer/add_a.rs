use super::Printer;
use crate::{Attr, Elem, Error, Node, Result, Spanned};

impl Printer<'_> {
    pub(super) fn add_a(&mut self, mut el: Elem) -> Result<()> {
        let href = if let Some(href) = el.remove_attr_opt("href") {
            href.into_value()?
        } else if let Some(ref_id) = el.remove_attr_opt("ref") {
            let ref_id = ref_id.value()?;

            self.refs.remove(&Some(ref_id.to_string())).ok_or_else(|| {
                Error::new(
                    format!("unknown ref: `{}`", ref_id.as_str()),
                    ref_id.span(),
                )
            })?
        } else {
            self.refs.remove(&None).ok_or_else(|| {
                Error::new("this link points nowhere", el.name.span())
            })?
        };

        if el.children.is_empty() {
            el.children.push(Node::Text(href.clone()));
        }

        el.attrs.push(Attr {
            name: Spanned::dummy("href".into()),
            value: Some(href),
        });

        self.add_ex(el)
    }
}
