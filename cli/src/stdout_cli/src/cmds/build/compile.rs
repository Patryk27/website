use anyhow::*;

use super::BuildContext;

pub async fn compile(ctxt: &mut BuildContext) -> Result<()> {
    println!();
    println!("[+] Loading posts");

    for post in ctxt.posts.find().await? {
        println!(" -  {}", post);

        let post = ctxt
            .posts
            .load(&post)
            .await
            .with_context(|| format!("Could not load post: {}", post))?;

        ctxt.site.add_post(post);
    }

    println!();
    println!("[+] Loading templates");

    let templates = ctxt
        .templates
        .load()
        .await
        .context("Could not load templates")?;

    println!();
    println!("[+] Compiling site");

    ctxt.site
        .compile(&templates)
        .await
        .context("Could not compile site")?;

    println!();
    println!("[+] Compiling theme");

    ctxt.theme
        .compile()
        .await
        .context("Could not compile theme")?;

    // @todo compile static resources

    Ok(())
}
