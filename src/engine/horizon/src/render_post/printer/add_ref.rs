use super::{Elem, Error, Node, Printer, Result, Spanned};

impl Printer<'_> {
    pub(super) fn add_ref(&mut self, mut el: Elem) -> Result<()> {
        let spanned_id = el.remove_attr_opt("id").and_then(|attr| attr.value);

        el.assert_no_attrs()?;

        if el.children.len() != 1 {
            return Err(Error::new(
                "ref must contain just one child",
                el.name.span(),
            ));
        }

        let url = if let Node::Text(url) = &el.children[0] {
            url
        } else {
            return Err(Error::new(
                "ref must contain a text node",
                el.children[0].span(),
            ));
        };

        if !url.starts_with('\n') {
            return Err(Error::new(
                "ref's link should start on the next line",
                url.span(),
            ));
        }

        let url = url.trim().to_owned();
        let span;
        let id;

        if let Some(spanned_id) = spanned_id {
            span = spanned_id.span();
            id = Some(spanned_id.into_inner());
        } else {
            span = el.name.span();
            id = None;
        }

        if let Some(prev_url) = self.refs.insert(id, Spanned::new(url, span)) {
            return Err(Error::new("ref overwrites another ref", span)
                .with_label("ref defined here before", prev_url.span()));
        }

        Ok(())
    }
}
