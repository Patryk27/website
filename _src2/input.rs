use std::collections::BTreeSet;

use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub tags: BTreeSet<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostMeta {
    pub title: String,
    pub tags: BTreeSet<String>,
    pub summary: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Tag {
    pub id: String,
    pub posts: BTreeSet<String>,
}

pub mod posts;