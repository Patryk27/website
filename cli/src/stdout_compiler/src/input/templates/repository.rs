use std::path::{Path, PathBuf};

use anyhow::*;
use tera::Tera;
use tokio::{task, fs};

use crate::input::Templates;

pub struct TemplatesRepository {
    dir: PathBuf,
}

impl TemplatesRepository {
    pub fn new(dir: &Path) -> Self {
        Self { dir: dir.into() }
    }

    pub async fn load(&mut self) -> Result<Templates> {
        let dir = fs::canonicalize(&self.dir).await.unwrap(); // @todo
        let dir = format!("{}/**/*.html", &dir.to_string_lossy());

        let tera = task::spawn_blocking(move || {
            Tera::new(&dir)
        }).await??;

        Ok(Templates::new(tera))
    }
}
