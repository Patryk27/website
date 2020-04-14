use std::path::PathBuf;

// @todo extract
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ArtifactId {
    Post {
        id: String,
    },

    Resource {
        id: String
    },

    Style,

    Tag {
        id: String,
    },
}

impl ArtifactId {
    pub fn path(&self) -> PathBuf {
        use ArtifactId::*;

        match self {
            Post { id } => {
                format!("post-{}", id)
            }

            Resource { id } => {
                format!("id-{}", id)
            }

            Style => {
                "style".into()
            }

            Tag { id } => {
                format!("tag-{}", id)
            }
        }.into()
    }
}

#[derive(Clone, Debug)]
pub struct Artifact {
    pub id: ArtifactId,
    pub content: String,
}

impl Artifact {
    pub fn post(id: String, content: String) -> Self {
        Self {
            id: ArtifactId::Post { id },
            content,
        }
    }
}