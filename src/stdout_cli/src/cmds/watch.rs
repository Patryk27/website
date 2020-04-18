use std::sync::mpsc;
use std::time::Duration;

use anyhow::{Context as _, Result};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

use crate::Context;

use self::event::*;

mod event;

pub fn watch(ctxt: &mut Context) -> Result<()> {
    let src = ctxt
        .src
        .canonicalize()
        .context("Could not canonicalize source path")?;

    println!();
    println!("[+] Watching for changes");
    println!();

    let (tx, events) = mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Duration::from_millis(250))
        .context("Could not create watcher")?;

    watcher
        .watch(&src, RecursiveMode::Recursive)
        .context("Could not start watching source for changes")?;

    let events = events
        .into_iter()
        .filter_map(|event| Event::convert(&src, event));

    for event in events {
        println!(" -  {}", event);
    }

    Ok(())
}
