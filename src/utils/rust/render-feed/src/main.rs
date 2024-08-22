use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use common::Env;
use rss::extension::atom::{self, AtomExtensionBuilder, Link};
use rss::validation::Validate;
use rss::{Channel, ChannelBuilder, GuidBuilder, Item, ItemBuilder};
use serde::Deserialize;
use std::io::Write;
use std::process::{Command, Stdio};

fn main() -> Result<()> {
    Env::with(main_ex)
}

fn main_ex(env: &mut Env) -> Result<()> {
    let mut website = String::new();

    env.stdin
        .read_to_string(&mut website)
        .context("couldn't read from stdin")?;

    let feed = Website::from_json(&website)?.into_feed();

    feed.validate().context("RSS validation failed")?;

    let feed = feed.to_string();

    // HACK validator.w3.org says it's best to have this kind of link, but
    //      apparently the `rss` crate always generates it as `<link href`
    //      instead of `<atom:link href`
    let feed = feed.replace("<link href", "<atom:link href");
    let feed = format_xml(&feed).context("couldn't format feed")?;

    env.stdout.write_all(feed.as_bytes())?;

    Ok(())
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Website {
    objects: Vec<Object>,
}

impl Website {
    fn from_json(s: &str) -> Result<Self> {
        serde_json::from_str(s).context("couldn't parse website")
    }

    fn into_feed(self) -> Channel {
        let atom = AtomExtensionBuilder::default()
            .link(Link {
                href: "https://pwy.io/feed.xml".into(),
                rel: "self".into(),
                mime_type: Some("application/rss+xml".into()),
                ..Default::default()
            })
            .build();

        let mut channel = ChannelBuilder::default()
            .namespace(("atom".into(), atom::NAMESPACE.into()))
            .title("pwy.io")
            .link("https://pwy.io")
            .description("pwy.io - throwing algorithms at problems and observing what happens!")
            .atom_ext(atom)
            .build();

        channel
            .items
            .extend(self.objects.into_iter().map(|post| post.into_feed()));

        channel
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
enum Object {
    Post {
        id: String,
        title: String,
        description: String,
        date: Date,
    },

    Talk {
        id: String,
        title: String,
        subtitle: Option<String>,
        date: Date,
    },
}

impl Object {
    fn into_feed(self) -> Item {
        match self {
            Object::Post {
                id,
                title,
                description,
                date,
            } => {
                let link = format!("https://pwy.io/posts/{}", id);

                let guid =
                    GuidBuilder::default().value(&link).permalink(true).build();

                ItemBuilder::default()
                    .title(title)
                    .link(link)
                    .description(description.trim().to_owned())
                    .guid(guid)
                    .pub_date(date.into_chrono().to_rfc2822())
                    .build()
            }

            Object::Talk {
                id,
                title,
                subtitle,
                date,
            } => {
                let link = format!("https://pwy.io/talks/{}", id);

                let guid =
                    GuidBuilder::default().value(&link).permalink(true).build();

                let description = if let Some(subtitle) = subtitle {
                    format!("{}: {}", title, subtitle)
                } else {
                    title.clone()
                };

                ItemBuilder::default()
                    .title(title)
                    .link(link)
                    .description(description)
                    .guid(guid)
                    .pub_date(date.into_chrono().to_rfc2822())
                    .build()
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Date {
    y: i32,
    m: u32,
    d: u32,
}

impl Date {
    fn into_chrono(self) -> DateTime<Utc> {
        Utc.from_utc_datetime(
            &NaiveDate::from_ymd_opt(self.y, self.m, self.d)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
    }
}

fn format_xml(xml: &str) -> Result<String> {
    let mut child = Command::new("xmllint")
        .args(["--encode", "utf-8", "--format", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("couldn't spawn xmllint")?;

    child
        .stdin
        .take()
        .unwrap()
        .write_all(xml.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();

    if output.status.success() {
        Ok(String::from_utf8(output.stdout).unwrap())
    } else {
        Err(anyhow!("xmllint returned a non-zero exit code"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::Asserter;
    use std::fs;
    use std::path::Path;
    use test_case::test_case;

    #[test_case("ok-smoke")]
    fn smoke(case: &str) {
        let dir = Path::new("src").join("tests").join(case);
        let stdin = fs::read_to_string(dir.join("given.stdin")).unwrap();
        let (result, stdout, _) = Env::mock(stdin, main_ex);

        result.unwrap();

        Asserter::new(dir)
            .assert("expected.stdout", stdout)
            .finish();
    }
}
