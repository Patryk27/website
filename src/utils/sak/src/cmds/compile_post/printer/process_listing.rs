use super::{Element, Message, MessageResult, Printer, Span};
use itertools::Itertools;
use std::fmt::Write as _;
use std::io::Write;
use std::process::{Command, Stdio};

impl Printer<'_> {
    pub(super) fn process_listing(&mut self, el: Element) -> MessageResult<()> {
        let listing = el.comment()?;
        let mut span = listing.span();

        let listing = listing.strip_prefix('\n').ok_or_else(|| {
            Message::new("expected comment to start with a newline", span)
        })?;

        span = Span::char(span.beg + 1);

        let indent = listing
            .chars()
            .take_while(|ch| *ch == ' ' || *ch == '=')
            .count();

        // ---

        let mut body = String::new();
        let mut highlights = Vec::new();

        for (line_idx, line) in listing.lines().enumerate() {
            let (line_indent, line_data) = if line.len() < indent {
                (line, "")
            } else {
                line.split_at(indent)
            };

            if !line_indent.bytes().all(|c| c == b' ' || c == b'=') {
                return Err(Message::new(
                    format!("invalid indentation: `{}`", line_indent),
                    span,
                ));
            }

            if line_indent.contains('=') {
                highlights.push(line_idx + 1);
            }

            span = Span::char(span.beg + line.len() + 1);

            // --

            if !body.is_empty() {
                _ = writeln!(&mut body);
            }

            let line = line_data.replace("-\\->", "-->");

            _ = write!(&mut body, "{}", line);
        }

        // ---

        let body = render_listing(&el, &body, &highlights)?;

        _ = write!(self.out, "{}", body.trim());

        Ok(())
    }
}

fn render_listing(
    el: &Element,
    body: &str,
    highlights: &[usize],
) -> MessageResult<String> {
    let mut lang = None;
    let mut class = None;

    for attr in &el.attrs {
        match attr.name.as_str() {
            "lang" => lang = Some(attr.value()?.as_str()),
            "class" => class = Some(attr.value()?.as_str()),

            _ => {
                return Err(Message::new(
                    "unknown attribute",
                    attr.name.span(),
                ));
            }
        }
    }

    // ---

    let mut command = Command::new("pygmentize");

    command.args(["-f", "html"]);

    if let Some(lang) = lang {
        command.args(["-l", lang]);
    }

    if let Some(class) = class {
        command.args(["-O", &format!("cssclass=listing {}", class)]);
    } else {
        command.args(["-O", "cssclass=listing"]);
    }

    if !highlights.is_empty() {
        command
            .args(["-O", &format!("hl_lines={}", highlights.iter().join(" "))]);
    }

    // ---

    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|err| {
            Message::new(
                format!(
                    "couldn't process listing: couldn't spawn pygmentize: {:?}",
                    err
                ),
                el.name.span(),
            )
        })?;

    child
        .stdin
        .take()
        .unwrap()
        .write_all(body.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();

    if output.status.success() {
        Ok(String::from_utf8(output.stdout).unwrap())
    } else {
        Err(Message::new(
            "couldn't proess listing: pygmentize returned a non-zero exit code",
            el.name.span(),
        ))
    }
}
