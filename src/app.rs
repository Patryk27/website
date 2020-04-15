use anyhow::Result;

use crate::{Posts, Site, Theme};

pub use self::{
    args::*,
    context::*,
};

mod args;
mod build;
mod context;
mod watch;

pub fn main(args: Args) -> Result<()> {
    println!("[+] Initializing");

    let posts_dir = args.src.join("posts");
    let theme_dir = args.src.join("theme");

    println!(" -  src: {}", args.src.to_string_lossy());
    println!(" -  dst: {}", args.dst.to_string_lossy());
    println!(" -  posts-dir: {}", posts_dir.to_string_lossy());
    println!(" -  theme-dir: {}", theme_dir.to_string_lossy());

    let mut ctxt = Context {
        posts: Posts::new(&posts_dir),
        site: Site::new(&args.dst)?,
        theme: Theme::new(&theme_dir, &args.dst),
        src: args.src,
    };

    build::build(&mut ctxt)?;

    println!();
    println!("Ok, site has been built");

    if args.watch {
        watch::watch(&mut ctxt)?;
    }

    Ok(())
}
