use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use rss::{
    extension::atom::{self, AtomExtensionBuilder, Link},
    validation::Validate,
    Channel, ChannelBuilder, GuidBuilder, Item, ItemBuilder,
};
use serde::Deserialize;
use std::io::{Read, Write};
use voca_rs::strip::strip_tags;

fn main() -> Result<()> {
    try_main(std::io::stdin().lock(), std::io::stdout().lock())
}

fn try_main(mut stdin: impl Read, mut stdout: impl Write) -> Result<()> {
    let mut website = String::new();

    stdin
        .read_to_string(&mut website)
        .context("Couldn't read website from stdin")?;

    let feed = Website::from_json(&website)?.into_feed();

    feed.validate().context("RSS validation failed")?;

    let feed = feed.to_string();

    // HACK validator.w3.org says it's best to have this kind of link, but
    //      apparently the `rss` crate always generates it as `<link href`
    //      instead of `<atom:link href`
    let feed = feed.replace("<link href", "<atom:link href");

    stdout.write_all(feed.as_bytes())?;

    Ok(())
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Website {
    posts: Vec<Post>,
}

impl Website {
    fn from_json(s: &str) -> Result<Self> {
        serde_json::from_str(s).context("Couldn't parse website")
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
            .extend(self.posts.into_iter().map(|post| post.into_feed()));

        channel
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
struct Post {
    id: String,
    title: String,
    summary: String,
    published_at: PublishedAt,
}

impl Post {
    fn into_feed(self) -> Item {
        let link = format!("https://pwy.io/{}", self.id);
        let guid = GuidBuilder::default().value(&link).permalink(true).build();

        ItemBuilder::default()
            .title(self.title)
            .link(link)
            .description(strip_tags(&self.summary).trim().to_owned())
            .guid(guid)
            .pub_date(self.published_at.into_chrono().to_rfc2822())
            .build()
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PublishedAt {
    y: i32,
    m: u32,
    d: u32,
}

impl PublishedAt {
    fn into_chrono(self) -> DateTime<Utc> {
        Utc.from_utc_date(&NaiveDate::from_ymd(self.y, self.m, self.d))
            .and_hms(0, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use pretty_assertions as pa;
    use std::io::Cursor;

    #[test]
    fn smoke() {
        let stdin = r#"
            {
              "posts": [
                {
                  "id": "hello-world",
                  "title": "Hello, World!",
                  "summary": "  This is my <i>first post</i> ~~ \"fancy!\"  ",
                  "publishedAt": {
                    "y": 2018,
                    "m": 3,
                    "d": 14
                  }
                }
              ]
            }
        "#;

        let actual_stdout = {
            let mut stdout = Vec::new();

            try_main(Cursor::new(stdin), &mut stdout).unwrap();

            String::from_utf8(stdout).unwrap()
        };

        let expected_stdout = {
            let stdout = r#"
                <?xml version="1.0" encoding="utf-8"?>
                <rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
                    <channel>
                        <title>pwy.io</title>
                        <link>https://pwy.io</link>
                        <description>pwy.io - throwing algorithms at problems and observing what happens!</description>
                        <atom:link href="https://pwy.io/feed.xml" rel="self" type="application/rss+xml"/>

                        <item>
                            <title>Hello, World!</title>
                            <link>https://pwy.io/hello-world</link>
                            <description><![CDATA[This is my first post ~~ "fancy!"]]></description>
                            <guid>https://pwy.io/hello-world</guid>
                            <pubDate>Wed, 14 Mar 2018 00:00:00 +0000</pubDate>
                        </item>
                    </channel>
                </rss>
            "#;

            stdout.lines().map(|line| line.trim()).join("")
        };

        pa::assert_eq!(expected_stdout, actual_stdout);
    }
}
