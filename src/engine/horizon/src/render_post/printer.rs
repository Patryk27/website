mod a;
mod code;
mod code_title;
mod hdr;
mod note;
mod p;
mod r#ref;
mod shdr;
mod toc;
mod video;

use super::{Attr, Element, Error, Node, Result, Span, Spanned};
use std::collections::BTreeMap;
use std::fmt::Write;

#[must_use = "call `.finish()` to emit warnings"]
pub struct Printer<'a> {
    out: &'a mut dyn Write,
    headers: Vec<Header>,
    refs: BTreeMap<Option<String>, Spanned<String>>,
}

impl<'a> Printer<'a> {
    pub fn new(out: &'a mut dyn Write) -> Self {
        Self {
            out,
            headers: Default::default(),
            refs: Default::default(),
        }
    }

    pub fn scan(&mut self, node: &Node) {
        if let Node::Element(el) = node
            && el.name.as_str() == "hdr"
        {
            self.scan_hdr(el);
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

            Node::Element(el) => {
                let el = el.into_inner();

                match el.name.as_str() {
                    "a" => self.add_a(el)?,
                    "code" => self.add_code(el)?,
                    "code-title" => self.add_code_title(el)?,
                    "hdr" => self.add_hdr(el)?,
                    "note" => self.add_note(el)?,
                    "p" => self.add_p(el)?,
                    "ref" => self.add_ref(el)?,
                    "shdr" => self.add_shdr(el)?,
                    "toc" => self.add_toc(el)?,
                    "video" => self.add_video(el)?,

                    #[rustfmt::skip]
                    "b"
                    | "blockquote"
                    | "br"
                    | "col"
                    | "colgroup"
                    | "del"
                    | "div"
                    | "figcaption"
                    | "figure"
                    | "hr"
                    | "i"
                    | "img"
                    | "input"
                    | "label"
                    | "li"
                    | "ol"
                    | "pre"
                    | "s"
                    | "script"
                    | "span"
                    | "style"
                    | "table"
                    | "tbody"
                    | "td"
                    | "th"
                    | "thead"
                    | "tr"
                    | "u"
                    | "ul"
                    => self.add_any(el)?,

                    tag => {
                        return Err(Error::new(
                            format!("unknown tag: `{tag}`"),
                            el.name.span(),
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    fn add_any(&mut self, el: Element) -> Result<()> {
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
                Err(Error::new(format!("unused ref: `{name}`"), url.span()))
            } else {
                Err(Error::new("unused ref", url.span()))
            };
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Header {
    id: String,
    name: Vec<Node>,
}
