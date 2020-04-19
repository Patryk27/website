use std::collections::BTreeSet;

use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Tag {
    pub id: String,
    pub posts: BTreeSet<String>,
}