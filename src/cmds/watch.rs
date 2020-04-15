use std::{fs, thread};
use std::path::PathBuf;

use anyhow::*;

pub fn watch(src: PathBuf, dst: PathBuf) -> Result<()> {
    println!("[+] Creating destination directory");

    if dst.exists() {
        bail!("Destination directory already exists, aborting");
    }

    fs::create_dir(&dst)
        .context("Could not create the destination directory")?;

    Ok(())
}
