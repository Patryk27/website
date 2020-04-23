use std::path::Path;

use anyhow::*;
use tokio::fs;

use crate::input::Post;

mod parse;
mod render;

pub async fn load(id: &str, path: &Path) -> Result<Post> {
    let id = id.to_owned();

    let content = fs::read_to_string(path)
        .await
        .with_context(|| format!("Could not access post file: {}", path.to_string_lossy()))?;

    let (header, summary, body) = parse::parse(&content)
        .context("Could not parse post")?;

    let summary = render::render(&summary)
        .await
        .context("Could not render post's summary")?;

    let body = render::render(&body)
        .await
        .context("Could not render post's body")?;

    Ok(Post {
        id,
        title: header.title,
        summary,
        body,
        tags: header.tags,
    })
}
