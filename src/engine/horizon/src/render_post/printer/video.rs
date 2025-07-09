use super::{Attr, Element, Printer, Result, Spanned};

impl Printer<'_> {
    pub(super) fn add_video(&mut self, mut el: Element) -> Result<()> {
        let src = el.remove_attr("src")?;
        let src = src.value()?;

        el.assert_no_attrs()?;

        self.add_any(Element {
            name: Spanned::dummy("video".into()),
            attrs: vec![
                Attr {
                    name: Spanned::dummy("src".into()),
                    value: Some(src.clone()),
                },
                Attr {
                    name: Spanned::dummy("controls".into()),
                    value: None,
                },
            ],
            children: Default::default(),
        })
    }
}
