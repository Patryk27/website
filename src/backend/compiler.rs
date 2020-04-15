use anyhow::*;
use anyhow::Context as _;
use html_minifier::minify;
use std::path::Path;
use std::process::Stdio;
use tera::{Context, Tera};
use tokio::fs;
use tokio::process::Command;

use crate::middleend::{Artifact, Post, Site};

pub async fn compile(src: &Path, site: &mut Site) -> Result<()> {
    let dst = src.join(".artifacts");

    // `Tera::new()` is a blocking call, but since at this point we're the only active `Future`, it doesn't pose any
    // threat to rest of the program

    compile_posts(&dst, &tera, site)
        .await
        .context("Could not compile posts")?;

    Ok(())
}

async fn compile_posts(dst: &Path, tera: &Tera, site: &mut Site) -> Result<()> {
    let ids: Vec<_> = site.posts
        .keys()
        .map(ToOwned::to_owned)
        .collect();

    for id in ids {
        let content = compile_post(tera, &site.posts[&id])
            .with_context(|| format!("Could not compile post `{}`", id))?;

        store_artifact(dst, site, Artifact::post(id, content))
            .await?;
    }

    Ok(())
}

fn compile_post(tera: &Tera, post: &Post) -> Result<String> {
    let mut ctxt = Context::new();

    ctxt.insert("post", post);

    let html = tera
        .render("post.html", &ctxt)
        .context("Could not render post")?;

    minify(html)
        .map_err(|err| anyhow!("Could not minify output: {}", err))
}

async fn store_artifact(dst: &Path, site: &mut Site, artifact: Artifact) -> Result<()> {
    let path = dst.join(artifact.id.path());

    site.artifacts.insert(
        artifact.id,
        path.clone(),
    );

    fs::write(&path, artifact.content)
        .await
        .with_context(|| format!("Could not store artifact: {:?}", path))
}