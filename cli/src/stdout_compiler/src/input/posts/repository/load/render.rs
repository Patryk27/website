use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::*;

pub fn render(body: &str) -> Result<String> {
    let mut cmd = Command::new("asciidoctor")
        .arg("--trace")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Could not spawn `asciidoctor`")?;

    let pipe = cmd.stdin
        .as_mut()
        .ok_or_else(|| anyhow!("Could not attach to `asciidoctor`'s stdin"))?;

    pipe.write(body.as_bytes())
        .context("Could not write to `asciidoctor`'s stdin")?;

    let cmd = cmd
        .wait_with_output()
        .context("Could not wait for `asciidoctor`")?;

    if !cmd.status.success() {
        bail!("{}", String::from_utf8_lossy(&cmd.stderr));
    }

    let content = String::from_utf8_lossy(&cmd.stdout)
        .into();

    Ok(content)
}

#[cfg(test)]
mod tests {
    use pretty_assertions as pa;

    use super::*;

    mod given_valid_document {
        use super::*;

        #[test]
        fn returns_valid_html() {
            let actual = render("hello *my dear* world").unwrap();
            let expected = "<div class=\"paragraph\">\n<p>hello <strong>my dear</strong> world</p>\n</div>\n";

            pa::assert_eq!(expected, actual);
        }
    }

    mod given_empty_document {
        use super::*;

        #[test]
        fn returns_empty_html() {
            let actual = render("").unwrap();
            let expected = "";

            pa::assert_eq!(expected, actual);
        }
    }

    mod given_invalid_document {
        use super::*;

        #[test]
        fn returns_error() {
            let actual = render("= A\n\n= B")
                .err()
                .unwrap()
                .to_string();

            let expected = "asciidoctor: ERROR: <stdin>: line 3: level 0 sections can only be used when doctype is book\n";

            pa::assert_eq!(expected, actual);
        }
    }
}