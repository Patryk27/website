use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

pub use self::repository::*;

mod repository;

#[derive(Clone, Debug, Serialize)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub body: String,
    pub tags: BTreeSet<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PostHeader {
    pub title: String,
    pub tags: BTreeSet<String>,
}
