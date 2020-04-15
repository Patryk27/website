use anyhow::*;

use crate::app;

fn build(ctxt: &mut Context) -> Result<()> {
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
    println!("[+] Rendering site");

    ctxt.site
        .render()
        .context("Could not render site")?;

    println!();
    println!("[+] Building theme");

    ctxt.theme
        .build()
        .context("Could not build theme")?;

    Ok(())
}
