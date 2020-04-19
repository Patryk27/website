use anyhow::*;

use super::BuildContext;

pub fn compile(ctxt: &mut BuildContext) -> Result<()> {
    println!();
    println!("[+] Loading posts");

    for post in ctxt.posts.find()? {
        println!(" -  {}", post);

        let post = ctxt
            .posts
            .load(&post)
            .with_context(|| format!("Could not load post: {}", post))?;

        ctxt.site.add_post(post);
    }

    println!();
    println!("[+] Loading templates");

    let templates = ctxt
        .templates
        .load()
        .context("Could not load templates")?;

    println!();
    println!("[+] Compiling site");

    ctxt.site
        .compile(&templates)
        .context("Could not compile site")?;

    println!();
    println!("[+] Compiling theme");

    ctxt.theme
        .compile()
        .context("Could not compile theme")?;

    Ok(())
}
