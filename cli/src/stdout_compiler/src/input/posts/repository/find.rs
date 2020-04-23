use std::path::Path;

use anyhow::*;
use tokio::task;

pub async fn find(dir: &Path) -> Result<Vec<String>> {
    let dir = dir.to_string_lossy().to_string();

    let posts: Result<_> = task::spawn_blocking(move || {
        let posts = glob::glob(&format!("{}/*.adoc", dir))
            .with_context(|| format!("Could not look for posts at `{}`", dir))?;

        let posts = posts
            .flat_map(|post| {
                let post = post
                    .ok()?
                    .file_stem()?
                    .to_str()?
                    .into();

                Some(post)
            })
            .collect();

        Ok(posts)
    }).await?;

    Ok(posts?)
}
