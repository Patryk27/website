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
    app::main(args)
        .unwrap();
}


