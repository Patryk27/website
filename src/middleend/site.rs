use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::middleend::{ArtifactId, Post};

#[derive(Debug, Default)]
pub struct Site {
    pub artifacts: HashMap<ArtifactId, PathBuf>,
    pub posts: HashMap<String, Post>,
    pub tags: HashSet<String>,
}
