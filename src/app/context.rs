use std::path::PathBuf;

use crate::{Posts, Site, Theme};

pub struct Context {
    pub src: PathBuf,
    pub posts: Posts,
    pub site: Site,
    pub theme: Theme,
}
