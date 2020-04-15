use anyhow::*;
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::middleend::{Post, Site};

pub async fn load(src: &Path) -> Result<Site> {
    let mut site = Site::default();

    load_posts(&mut site, src.join("posts"))
        .await
        .context("Could not load posts")?;

    load_resources(&mut site, src.join("resources"))
        .await
        .context("Could not compile resources")?;

    Ok(site)
}

async fn load_posts(site: &mut Site, src: PathBuf) -> Result<()> {
    let mut posts = fs::read_dir(&src).await
        .with_context(|| format!("Could not read directory: {:?}", src))?;

    while let Some(post) = posts.next_entry().await? {
        let post = post.path();

        let post = load_post(&post)
            .await
            .with_context(|| format!("Could not compile post `{:?}`", post.file_stem().unwrap()))?;

        site.posts.insert(post.id.clone(), post);
    }

    Ok(())
}

async fn load_post(src: &Path) -> Result<Post> {
    let id = src
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .into();

    let content = fs::read_to_string(src).await?;

    Ok(Post {
        id,
        title: "foo".into(),
        content,
        tags: Default::default(),
    })
}

async fn load_resources(site: &mut Site, src: PathBuf) -> Result<()> {
    Ok(())
}
