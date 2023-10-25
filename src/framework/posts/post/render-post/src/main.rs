//! TODO needs solid refactoring

#![feature(exit_status_error)]
#![feature(try_blocks)]

use anyhow::{anyhow, bail, Context, Result};
use itertools::Itertools;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

fn main() -> Result<()> {
    let mut input = String::new();

    std::io::stdin()
        .read_to_string(&mut input)
        .context("Couldn't read post from stdin")?;

    let output = render(&input)?;

    print!("{}", output);

    Ok(())
}

fn render(input: &str) -> Result<String> {
    let mut input = input
        .lines()
        .enumerate()
        .map(|(line_idx, line)| (line_idx + 1, line));

    let mut output = String::new();

    while let Some((line_idx, line)) = input.next() {
        if let Some(attrs) = line.trim().strip_prefix("<listing") {
            let attrs: BTreeMap<_, _> = attrs
                .strip_suffix('>')
                .context("Tag is missing ending `>`")?
                .split(" ")
                .map(|attr| attr.trim())
                .filter(|attr| !attr.is_empty())
                .map(|attr| {
                    let result: Result<_> = try {
                        let kv: Vec<_> = attr.splitn(2, "=").collect();

                        let (k, v) = if let &[k, v] = &kv[..] {
                            (k, v)
                        } else {
                            bail!("Missing `=`");
                        };

                        let v = v
                            .strip_prefix('"')
                            .context("Value is missing beginning `\"`")?;

                        let v = v
                            .strip_suffix('"')
                            .context("Value is missing ending `\"`")?;

                        (k, v)
                    };

                    result.with_context(|| {
                        anyhow!("Invalid key-value syntax: {}", attr)
                    })
                })
                .collect::<Result<_, _>>()
                .with_context(|| format!("At line {}", line_idx))?;

            let indent = loop {
                let (_, listing_line) = input
                    .next()
                    .ok_or_else(|| anyhow!("Unterminated listing"))
                    .with_context(|| format!("At line {}", line_idx))?;

                if listing_line.trim() == "<!--" {
                    break listing_line
                        .bytes()
                        .take_while(|&n| n == b' ')
                        .count();
                }
            };

            let mut body = String::new();
            let mut highlights = Vec::new();

            loop {
                let (listing_line_idx, listing_line) = input
                    .next()
                    .ok_or_else(|| anyhow!("Unterminated listing"))
                    .with_context(|| format!("At line {}", line_idx))?;

                match listing_line.trim() {
                    "-->" => {
                        continue;
                    }
                    "</listing>" => {
                        break;
                    }
                    val if val.is_empty() => {
                        _ = writeln!(&mut body);
                        continue;
                    }
                    _ => (),
                }

                let (listing_line_indent, listing_line) =
                    if listing_line.len() < indent + 2 {
                        (listing_line, "")
                    } else {
                        listing_line.split_at(indent + 2)
                    };

                if !listing_line_indent.bytes().all(|c| c == b' ' || c == b'=')
                {
                    return Err(anyhow!("Misindented listing")).with_context(
                        || format!("At line {}", listing_line_idx),
                    );
                }

                if listing_line_indent.contains('=') {
                    highlights.push(listing_line_idx - line_idx - 1);
                }

                if !body.is_empty() {
                    _ = writeln!(&mut body);
                }

                let listing_line = listing_line.replace("-\\->", "-->");

                _ = write!(&mut body, "{}", listing_line);
            }

            let listing = render_listing(&attrs, &highlights, &body)
                .context("Invalid listing")
                .with_context(|| format!("At line {}", line_idx))?;

            let indent = indent - 2;

            for (idx, listing_line) in listing.lines().enumerate() {
                if idx == 0 && indent > 0 {
                    _ = write!(&mut output, "{:indent$}", " ", indent = indent);
                }

                _ = writeln!(&mut output, "{}", listing_line);
            }
        } else {
            _ = writeln!(&mut output, "{}", line);
        }
    }

    Ok(output)
}

fn render_listing(
    attrs: &BTreeMap<&str, &str>,
    highlights: &[usize],
    body: &str,
) -> Result<String> {
    let mut command = Command::new("pygmentize");

    if let Some(lang) = attrs.get("lang") {
        command.args(["-l", lang]);
    }

    if let Some(class) = attrs.get("class") {
        command.args(["-O", &format!("cssclass=listing {}", class)]);
    } else {
        command.args(["-O", "cssclass=listing"]);
    }

    if !highlights.is_empty() {
        command
            .args(["-O", &format!("hl_lines={}", highlights.iter().join(" "))]);
    }

    let mut child = command
        .args(["-f", "html"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("Couldn't spawn `pygmentize`")?;

    child
        .stdin
        .take()
        .context("Couldn't open stdin")?
        .write_all(body.as_bytes())
        .context("Couldn't write listing into stdin")?;

    let output = child.wait_with_output().context("Couldn't read stdout")?;

    output.status.exit_ok()?;

    String::from_utf8(output.stdout)
        .context("Stdout is not a valid UTF-8 string")
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions as pa;
    use test_case::test_case;

    struct TestCase {
        given: &'static str,
        expected: &'static str,
    }

    const TEST_LISTING_OK: TestCase = TestCase {
        given: indoc! {r#"
            <listing>
              <!--
                fn main() {
                    println!("First Listing");
                }
              -->
            </listing>

            <aside class="note">
              <listing lang="rust">
                <!--
                  fn main() {
                =     println!("Second Listing");
                = }
                -->
              </listing>
            </aside>

            <listing lang="python" class="listing-invalid">
              <!--
                def main():

                    print "Third Listing"

              -->
            </listing>
        "#},
        expected: indoc! {r#"
            <div class="listing"><pre><span></span>fn main() {
                println!(&quot;First Listing&quot;);
            }
            </pre></div>

            <aside class="note">
              <div class="listing"><pre><span></span><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w"></span>
            <span class="hll"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&quot;Second Listing&quot;</span><span class="p">);</span><span class="w"></span>
            </span><span class="hll"><span class="p">}</span><span class="w"></span>
            </span></pre></div>
            </aside>

            <div class="listing listing-invalid"><pre><span></span><span class="k">def</span> <span class="nf">main</span><span class="p">():</span>

                <span class="nb">print</span> <span class="s2">&quot;Third Listing&quot;</span>
            </pre></div>
        "#},
    };

    #[test_case(TEST_LISTING_OK)]
    fn test(case: TestCase) {
        let actual = render(case.given)
            .map_err(|err| format!("{:?}", err))
            .unwrap_or_else(|err| err);

        pa::assert_eq!(case.expected, actual);
    }
}
