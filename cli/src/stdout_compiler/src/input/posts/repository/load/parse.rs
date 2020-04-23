use anyhow::*;
use nom::{Err, IResult};
use nom::bytes::complete::{tag, take_until};
use nom::error::{context, convert_error, VerboseError};
use nom::sequence::tuple;

use crate::input::PostHeader;

pub fn parse(content: &str) -> Result<(PostHeader, String, String)> {
    let result = tuple((
        context("header", extract_header),
        context("summary", extract_summary),
    ))(content);

    let (header, summary, body) = match result {
        Ok((body, (header, summary))) => {
            (header, summary, body)
        }

        Err(Err::Error(err)) | Err(Err::Failure(err)) => {
            return Err(anyhow!("{}", convert_error(content, err)));
        }

        Err(Err::Incomplete(_)) => {
            unreachable!()
        }
    };

    let header = serde_yaml::from_str(header)
        .context("Could not parse header")?;

    let summary = summary.to_owned();

    let body = body.to_owned();

    Ok((header, summary, body))
}

fn extract_header(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
    let (i, (_, header, _)) = tuple((
        context("header's opening tag", tag("---\n")),
        context("header's contents", take_until("---\n")),
        context("header's closing tag", tag("---\n")),
    ))(i)?;

    Ok((i, header.trim()))
}

fn extract_summary(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
    const TAG: &str = "\n<!-- more -->\n";

    let (body, (summary, _)) = tuple((
        context("summary's contents", take_until(TAG)),
        context("summary's tag", tag(TAG)),
    ))(i)?;

    Ok((body.trim(), summary.trim()))
}

#[cfg(test)]
mod tests {
    use pretty_assertions as pa;

    use super::*;

    mod given_valid_document {
        use super::*;

        #[test]
        fn returns_parsed_document() {
            let (header, summary, body) = parse(&[
                "---",
                "title: 'Hello, World!'",
                "tags: [ \'foo\', \'bar\' ]",
                "---",
                "",
                "first line of summary",
                "second line of summary",
                "",
                "<!-- more -->",
                "",
                "first line of body",
                "second line of body",
            ]).unwrap();

            pa::assert_eq!("Hello, World!", &header.title);

            assert_eq!(2, header.tags.len());
            assert!(header.tags.contains("foo"));
            assert!(header.tags.contains("bar"));

            pa::assert_eq!("first line of summary\nsecond line of summary", &summary);
            pa::assert_eq!("first line of body\nsecond line of body", &body);
        }
    }

    // @todo
    // mod given_document_without_header {
    //     use super::*;
    //
    //     #[test]
    //     fn returns_error() {
    //         let err = parse(&[
    //             "This is the summary",
    //             "",
    //             "<!-- more -->",
    //             "",
    //             "This is the body",
    //         ]).err().unwrap().to_string();
    //
    //         assert!(err.starts_with("Could not parse post"));
    //     }
    // }
    //
    // mod given_document_without_summary {
    //     use super::*;
    //
    //     #[test]
    //     fn returns_error() {
    //         let err = parse(&[
    //             "---",
    //             "title: 'Hello, World!'",
    //             "tags: [ \'foo\', \'bar\' ]",
    //             "---",
    //             "",
    //             "This is the body",
    //         ]).err().unwrap().to_string();
    //
    //         assert!(err.starts_with("Could not parse post"));
    //     }
    // }

    mod given_document_without_body {
        use super::*;

        #[test]
        fn returns_parsed_document_with_empty_body() {
            let (header, summary, body) = parse(&[
                "---",
                "title: 'Hello, World!'",
                "tags: [ \'foo\', \'bar\' ]",
                "---",
                "",
                "This is the summary",
                "",
                "<!-- more -->",
                "",
            ]).unwrap();

            pa::assert_eq!("Hello, World!", &header.title);

            assert_eq!(2, header.tags.len());
            assert!(header.tags.contains("foo"));
            assert!(header.tags.contains("bar"));

            pa::assert_eq!("This is the summary", &summary);
            pa::assert_eq!("", &body);
        }
    }

    fn parse(lines: &[&str]) -> Result<(PostHeader, String, String)> {
        super::parse(&lines.join("\n"))
    }
}