use std::path::{Path, PathBuf};

use anyhow::*;

use crate::Post;

mod find;
mod load;

pub struct Posts {
    dir: PathBuf,
}

impl Posts {
    pub fn new(dir: &Path) -> Self {
        Self { dir: dir.into() }
    }

    pub fn find(&self) -> Result<Vec<String>> {
        find::find_posts(&self.dir)
    }

    pub fn load(&self, id: &str) -> Result<Post> {
        let path = self.dir
            .join(id)
            .with_extension("adoc");

        load::load_post(id, &path)
    }
}