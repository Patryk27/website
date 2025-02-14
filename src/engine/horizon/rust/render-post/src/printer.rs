mod add_a;
mod add_h2;
mod add_listing;
mod add_listing_title;
mod add_note;
mod add_p;
mod add_ref;
mod add_video;

use crate::{Elem, Error, Node, Result, Spanned};
use std::collections::BTreeMap;
use std::fmt::Write;

#[must_use = "call `.finish()` to emit warnings"]
pub struct Printer<'a> {
    out: &'a mut dyn Write,
    refs: BTreeMap<Option<String>, Spanned<String>>,
}

impl<'a> Printer<'a> {
    pub fn new(out: &'a mut dyn Write) -> Self {
        Self {
            out,
            refs: Default::default(),
        }
    }

    pub fn add(&mut self, node: Node) -> Result<()> {
        match node {
            Node::Text(text) => {
                _ = write!(self.out, "{}", *text);
            }

            Node::Comment(_) => {
                //
            }

            Node::Elem(el) => {
                let el = el.into_inner();

                match el.name.as_str() {
                    "a" => self.add_a(el)?,
                    "h2" => self.add_h2(el)?,
                    "listing" => self.add_listing(el)?,
                    "listing-title" => self.add_listing_title(el)?,
                    "note" => self.add_note(el)?,
                    "p" => self.add_p(el)?,
                    "ref" => self.add_ref(el)?,
                    "video" => self.add_video(el)?,

                    tag @ "h1" => {
                        return Err(Error::new(
                            format!("forbidden tag: `{}`", tag),
                            el.name.span(),
                        ));
                    }

                    _ => self.add_ex(el)?,
                }
            }
        }

        Ok(())
    }

    fn add_ex(&mut self, el: Elem) -> Result<()> {
        _ = write!(self.out, "<{}", *el.name);

        for attr in &el.attrs {
            if let Some(attr_val) = &attr.value {
                _ = write!(self.out, r#" {}="{}""#, *attr.name, **attr_val);
            } else {
                _ = write!(self.out, r#" {}"#, *attr.name);
            }
        }

        if ["br", "col", "hr", "img", "input"].contains(&el.name.as_str()) {
            if !el.children.is_empty() {
                return Err(Error::new(
                    format!("{} cannot contain children", el.name.as_str()),
                    el.name.span(),
                ));
            }

            _ = write!(self.out, " />");
        } else {
            _ = write!(self.out, ">");

            for el in el.children {
                self.add(el)?;
            }

            _ = write!(self.out, "</{}>", *el.name);
        }

        Ok(())
    }

    pub fn finish(self) -> Result<()> {
        if let Some((name, url)) = self.refs.first_key_value() {
            return if let Some(name) = name {
                Err(Error::new(format!("unused ref: `{}`", name), url.span()))
            } else {
                Err(Error::new("unused ref", url.span()))
            };
        }

        Ok(())
    }
}
