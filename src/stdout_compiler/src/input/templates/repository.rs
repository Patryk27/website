use std::path::{Path, PathBuf};

use anyhow::*;
use tera::Tera;

use crate::input::Templates;

pub struct TemplatesRepository {
    dir: PathBuf,
}

impl TemplatesRepository {
    pub fn new(dir: &Path) -> Self {
        Self { dir: dir.into() }
    }

    pub fn load(&mut self) -> Result<Templates> {
        let dir = &format!("{}/**/*.html", self.dir.to_string_lossy());
        let tera = Tera::new(dir)?;

        Ok(Templates::new(tera))
    }
}
