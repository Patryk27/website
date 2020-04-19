use std::path::PathBuf;

use anyhow::*;

use stdout_compiler::input::{PostsRepository, TemplatesRepository};
use stdout_compiler::output::{SiteCompiler, ThemeCompiler};

use self::context::*;

mod compile;
mod context;
mod watch;

pub fn build(src: PathBuf, dst: PathBuf, watch: bool) -> Result<()> {
    println!("[+] Building");
    println!(" -  src: {}", src.to_string_lossy());
    println!(" -  dst: {}", dst.to_string_lossy());

    let posts = PostsRepository::new(
        &src.join("posts")
    );

    let templates = TemplatesRepository::new(
        &src.join("templates")
    );

    let theme = ThemeCompiler::new(
        &src.join("theme"),
        &dst,
    );

    let site = SiteCompiler::new(&dst)?;

    let mut ctxt = BuildContext {
        src,
        posts,
        templates,
        theme,
        site,
    };

    compile::compile(&mut ctxt)?;

    println!();
    println!("Ok, site has been built");

    if watch {
        watch::watch(&mut ctxt)?;
    }

    Ok(())
}