use super::{Elem, Error, Node, Printer, Result, Span};
use itertools::Itertools;
use std::fmt::Write as _;
use std::io::Write;
use std::process::{Command, Stdio};

impl Printer<'_> {
    pub(super) fn add_code(&mut self, el: Elem) -> Result<()> {
        if el.children.iter().any(|el| matches!(el, Node::Comment(_))) {
            self.add_code_block(el)
        } else {
            self.add_any(el)
        }
    }

    fn add_code_block(&mut self, el: Elem) -> Result<()> {
        let mut code = None;

        for el in &el.children {
            match el {
                Node::Text(text) if text.trim().is_empty() => {
                    //
                }
                Node::Comment(comment) if code.is_none() => {
                    code = Some(comment);
                }
                _ => {
                    return Err(Error::new("unexpected node", el.span()));
                }
            }
        }

        let code = code.unwrap();
        let mut span = code.span();

        // ---

        let code = code.strip_prefix('\n').ok_or_else(|| {
            Error::new("expected comment to start with a newline", span)
        })?;

        span = Span::char(span.start + 1);

        let indent = code
            .chars()
            .take_while(|ch| *ch == ' ' || *ch == '=')
            .count();

        // ---

        let mut body = String::new();
        let mut highlights = Vec::new();

        for (line_idx, line) in code.lines().enumerate() {
            let (line_indent, line_data) = if line.len() < indent {
                (line, "")
            } else {
                line.split_at(indent)
            };

            if !line_indent.bytes().all(|c| c == b' ' || c == b'=') {
                return Err(Error::new(
                    format!("invalid indentation: `{line_indent}`"),
                    span,
                ));
            }

            if line_indent.contains('=') {
                highlights.push(line_idx + 1);
            }

            span = Span::char(span.start + line.len() + 1);

            // --

            if !body.is_empty() {
                _ = writeln!(&mut body);
            }

            let line = line_data.replace("-\\->", "-->");

            _ = write!(&mut body, "{line}");
        }

        // ---

        let body = render(&el, &body, &highlights)?;

        _ = write!(self.out, "{}", body.trim());

        Ok(())
    }
}

fn render(el: &Elem, body: &str, highlights: &[usize]) -> Result<String> {
    let mut lang = None;
    let mut class = None;

    for attr in &el.attrs {
        match attr.name.as_str() {
            "lang" => lang = Some(attr.value()?.as_str()),
            "class" => class = Some(attr.value()?.as_str()),

            _ => {
                return Err(Error::new("unknown attribute", attr.name.span()));
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
        command.args(["-O", &format!("cssclass=code {class}")]);
    } else {
        command.args(["-O", "cssclass=code"]);
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
            Error::new(
                format!(
                    "couldn't process code: couldn't spawn pygmentize: {err:?}",
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
        Err(Error::new(
            "couldn't process code: pygmentize returned a non-zero exit code",
            el.name.span(),
        ))
    }
}
