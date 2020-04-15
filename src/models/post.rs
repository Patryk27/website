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
