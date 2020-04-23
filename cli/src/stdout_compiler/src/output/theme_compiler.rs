use std::path::{Path, PathBuf};
use std::process::Stdio;

use anyhow::*;
use tokio::process::Command;
use tokio::time;

pub struct ThemeCompiler {
    src: PathBuf,
    dst: PathBuf,
}

impl ThemeCompiler {
    pub fn new(src: &Path, dst: &Path) -> Self {
        Self {
            src: src.into(),
            dst: dst.into(),
        }
    }

    pub async fn compile(&mut self) -> Result<()> {
        let cmd = Command::new("sass")
            .arg("-C")
            .arg("--sourcemap=none")
            .arg(self.src.join("theme.scss"))
            .arg(self.dst.join("theme.css"))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .context("Could not spawn `sass`")?;

        let result = time::timeout(time::Duration::from_secs(10), async {
            let out = cmd
                .wait_with_output()
                .await
                .context("Could not wait for `sass`")?;

            if out.status.success() {
                Ok(())
            } else {
                bail!("{}", String::from_utf8_lossy(&out.stderr));
            }
        }).await;

        match result {
            Ok(result) => result,
            Err(_) => bail!("`sass` timed out"),
        }
    }
}