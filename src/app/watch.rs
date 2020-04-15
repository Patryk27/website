use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Duration;

use anyhow::*;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use pathdiff::diff_paths;

use crate::app;

#[derive(Debug)]
enum Event {
    PostCreated {
        id: String,
    },

    PostUpdated {
        id: String,
    },

    PostDeleted {
        id: String,
    },

    ThemeUpdated,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Event::*;

        match self {
            PostCreated { id } => {
                write!(f, "post created: `{}`", id)
            }

            PostUpdated { id } => {
                write!(f, "post updated: `{}`", id)
            }

            PostDeleted { id } => {
                write!(f, "post deleted: `{}`", id)
            }

            ThemeUpdated => {
                write!(f, "theme updated")
            }
        }
    }
}

pub fn watch(ctxt: &mut app::Context) -> Result<()> {
    let src = src
        .canonicalize()
        .context("Could not canonicalize source path")?;

    super::build(&src, &dst)?;

    println!();
    println!("[+] Watching for changes");
    println!();

    let (tx, rx) = mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Duration::from_millis(250))
        .context("Could not create watcher")?;

    watcher
        .watch(&src, RecursiveMode::Recursive)
        .context("Could not start watching directory for changes")?;

    let events = rx
        .into_iter()
        .filter_map(|event| {
            match event {
                DebouncedEvent::Write(path) => {
                    let path = diff_paths(&path, &src)?;

                    if path.starts_with("posts") {
                        let id = path
                            .file_stem()?
                            .to_string_lossy()
                            .into();

                        Some(Event::PostUpdated { id })
                    } else {
                        None
                    }
                }

                _ => {
                    None
                }
            }
        });

    for event in events {
        println!(" -  {}", event);

        match event {
            Event::PostCreated { id } => {
                compiler.add_post();
            }

            Event::PostDeleted { id } => {}
        }
    }

    println!("[+] Stopping");

    Ok(())
}
