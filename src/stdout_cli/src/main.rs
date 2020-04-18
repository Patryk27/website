use anyhow::Result;

use stdout_compiler::input::{PostsRepository, TemplatesRepository};
use stdout_compiler::output::{SiteCompiler, ThemeCompiler};

pub use self::{
    args::*,
    cmds::*,
    context::*,
};

mod args;
mod cmds;
mod context;

#[paw::main]
fn main(args: Args) -> Result<()> {
    println!("[+] Initializing");
    println!(" -  src: {}", args.src.to_string_lossy());
    println!(" -  dst: {}", args.dst.to_string_lossy());

    let posts = PostsRepository::new(
        &args.src.join("posts")
    );

    let templates = TemplatesRepository::new(
        &args.src.join("templates")
    );

    let theme = ThemeCompiler::new(
        &args.src.join("theme"),
        &args.dst,
    );

    let site = SiteCompiler::new(&args.dst)?;

    let mut ctxt = Context {
        src: args.src,
        posts,
        templates,
        theme,
        site,
    };

    cmds::build(&mut ctxt)?;

    println!();
    println!("Ok, site has been built");

    if args.watch {
        cmds::watch(&mut ctxt)?;
    }

    Ok(())
}
