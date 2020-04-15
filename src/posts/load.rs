use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::*;

use crate::{Post, PostMeta};

pub fn load_post(id: &str, path: &Path) -> Result<Post> {
    let content = fs::read_to_string(&path)
        .with_context(|| format!("Could not open post for reading: {}", path.to_string_lossy()))?;

    let (meta, content) = strip_meta(content)
        .context("Could not read post's meta-data")?;

    let content = render(content)
        .context("Could not render post")?;

    Ok(Post {
        id: id.to_owned(),
        title: meta.title,
        summary: meta.summary,
        content,
        tags: meta.tags,
    })
}

fn strip_meta(content: String) -> Result<(PostMeta, String)> {
    let meta = PostMeta {
        title: "test".into(),
        tags: Default::default(),
        summary: "summary".into(),
    };

    Ok((meta, content))
}

fn render(content: String) -> Result<String> {
    let mut cmd = Command::new("asciidoctor")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("Could not spawn `asciidoctor`")?;

    let pipe = cmd.stdin
        .as_mut()
        .ok_or_else(|| anyhow!("Could not attach to `asciidoctor`'s stdin"))?;

    pipe.write(content.as_bytes())
        .context("Could not write to `asciidoctor`'s stdin")?;

    let cmd = cmd
        .wait_with_output()
        .context("Could not wait for `asciidoctor`")?;

    if !cmd.status.success() {
        bail!("{}", String::from_utf8_lossy(&cmd.stderr));
    }

    let content = String::from_utf8_lossy(&cmd.stdout)
        .into();

    Ok(content)
}