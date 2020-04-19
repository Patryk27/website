use std::fs;
use std::path::Path;

use anyhow::*;

use crate::input::Post;

mod parse;
mod render;

pub fn load(id: &str, path: &Path) -> Result<Post> {
    let id = id.to_owned();

    let content = fs::read_to_string(&path)
        .with_context(|| format!("Could not access post file: {}", path.to_string_lossy()))?;

    let (header, summary, body) = parse::parse(&content)
        .context("Could not parse post")?;

    let summary = render::render(&summary)
        .context("Could not render post's summary")?;

    let body = render::render(&body)
        .context("Could not render post's body")?;

    Ok(Post {
        id,
        title: header.title,
        summary,
        body,
        tags: header.tags,
    })
}
