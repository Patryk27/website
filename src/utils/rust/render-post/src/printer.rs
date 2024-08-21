#![allow(clippy::needless_pub_self)]

mod process_a;
mod process_generic;
mod process_header;
mod process_listing;
mod process_listing_title;
mod process_note;
mod process_ref;

pub(self) use super::{
    Attr, Element, Message, MessageResult, Node, Span, Spanned,
};
use std::collections::BTreeMap;
use std::fmt::Write;

#[must_use = "call `.finish()` to emit warnings"]
pub struct Printer<'a> {
    out: &'a mut dyn Write,
    refs: BTreeMap<Option<String>, (String, Span)>,
}

impl<'a> Printer<'a> {
    pub fn new(out: &'a mut dyn Write) -> Self {
        Self {
            out,
            refs: Default::default(),
        }
    }

    pub fn process(&mut self, node: Node) -> MessageResult<()> {
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
                    "a" => self.process_a(el)?,
                    "header" => self.process_header(el)?,
                    "listing" => self.process_listing(el)?,
                    "listing-title" => self.process_listing_title(el)?,
                    "note" => self.process_note(el)?,
                    "ref" => self.process_ref(el)?,
                    _ => self.process_generic(el)?,
                }
            }
        }

        Ok(())
    }

    pub fn finish(self) -> MessageResult<()> {
        if let Some((name, (_, span))) = self.refs.first_key_value() {
            return if let Some(name) = name {
                Err(Message::new(format!("unused ref: `{}`", name), *span))
            } else {
                Err(Message::new("unused ref", *span))
            };
        }

        Ok(())
    }
}
