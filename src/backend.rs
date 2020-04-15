use anyhow::*;
use std::path::PathBuf;

use crate::middleend::Site;

mod compiler;
mod loader;

pub async fn build(src: PathBuf) -> Result<Site> {
    let mut site = loader::load(&src).await
        .context("Could not load site")?;

    compiler::compile(&src, &mut site).await
        .context("Could not compile site")?;

    Ok(site)
}