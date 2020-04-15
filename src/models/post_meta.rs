use std::collections::BTreeSet;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct PostMeta {
    pub title: String,
    pub tags: BTreeSet<String>,
}
