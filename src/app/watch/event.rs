use std::fmt;

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
