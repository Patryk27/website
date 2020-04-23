use std::path::{Path, PathBuf};

use anyhow::*;

use crate::input::Post;

mod find;
mod load;

pub struct PostsRepository {
    dir: PathBuf,
}

impl PostsRepository {
    pub fn new(dir: &Path) -> Self {
        Self { dir: dir.into() }
    }

    pub async fn find(&self) -> Result<Vec<String>> {
        find::find(&self.dir)
            .await
    }

    pub async fn load(&self, id: &str) -> Result<Post> {
        let path = self.dir
            .join(id)
            .with_extension("adoc");

        load::load(id, &path)
            .await
    }
}