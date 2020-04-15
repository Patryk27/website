use std::path::PathBuf;

use anyhow::*;
use anyhow::Context as _;
use structopt::StructOpt;

pub use self::{
    post::*,
    post_meta::*,
    posts::*,
    site::*,
    tag::*,
    theme::*,
};

mod app;
mod post;
mod post_meta;
mod posts;
mod site;
mod tag;
mod theme;

#[paw::main]
fn main(args: app::Args) {
    try_main(args)
        .unwrap();
}


