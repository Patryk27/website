use std::collections::BTreeSet;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: BTreeSet<String>,
}
