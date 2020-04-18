use std::fmt;
use std::path::Path;

use notify::DebouncedEvent;
use pathdiff::diff_paths;

#[derive(Debug)]
pub enum Event {
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

impl Event {
    pub fn convert(src: &Path, event: DebouncedEvent) -> Option<Self> {
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
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Event::*;

        match self {
            PostCreated { id } => {
                write!(f, "Post created: `{}`", id)
            }

            PostUpdated { id } => {
                write!(f, "Post updated: `{}`", id)
            }

            PostDeleted { id } => {
                write!(f, "Post deleted: `{}`", id)
            }

            ThemeUpdated => {
                write!(f, "Theme updated")
            }
        }
    }
}
