use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::*;

pub struct Theme {
    src: PathBuf,
    dst: PathBuf,
}

impl Theme {
    pub fn new(src: &Path, dst: &Path) -> Self {
        Self {
            src: src.into(),
            dst: dst.into(),
        }
    }

    pub fn build(&mut self) -> Result<()> {
        let cmd = Command::new("sass")
            .arg("-C")
            .arg("--sourcemap=none")
            .arg(self.src.join("theme.scss"))
            .arg(self.dst.join("theme.css"))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Could not spawn `sass`")?;

        let cmd = cmd
            .wait_with_output()
            .context("Could not wait for `sass`")?;

        if !cmd.status.success() {
            bail!("{}", String::from_utf8_lossy(&cmd.stderr));
        }

        Ok(())
    }
}