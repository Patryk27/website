use super::Printer;
use crate::{Attr, Elem, Result, Spanned};

impl Printer<'_> {
    pub(super) fn add_video(&mut self, mut el: Elem) -> Result<()> {
        let src = el.remove_attr("src")?;
        let src = src.value()?;

        el.assert_no_attrs()?;

        self.add_ex(Elem {
            name: Spanned::dummy("video".into()),
            attrs: vec![Attr {
                name: Spanned::dummy("src".into()),
                value: Some(src.clone()),
            }],
            children: Default::default(),
        })
    }
}
