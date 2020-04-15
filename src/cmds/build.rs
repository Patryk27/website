use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::*;
use chrono::Utc;
use tera::Tera;

use crate::models::{Post, Tag};

pub fn build(src: PathBuf, dst: PathBuf) -> Result<()> {
    println!("[+] Creating skeleton");

    create_skeleton(&dst)
        .context("Could not create skeleton")?;

    println!();
    println!("[+] Building site");

    build_site(&src, &dst)
        .context("Could not build site")?;

    println!();
    println!("[+] Building theme");

    build_theme(&src, &dst)
        .context("Could not build theme")?;

    println!();
    println!("[+] Completed");

    Ok(())
}

fn create_skeleton(dst: &Path) -> Result<()> {
    if dst.exists() {
        bail!("Directory already exists, aborting");
    }

    fs::create_dir(&dst)
        .context("Could not create the main directory")?;

    fs::write(dst.join(".gitignore"), "*")
        .context("Could not create `.gitignore`")?;

    fs::write(dst.join(".timestamp"), Utc::now().to_rfc2822())
        .context("Could not create `.timestamp`")?;

    fs::create_dir(dst.join("assets"))
        .context("Could not create the `assets` directory")?;

    fs::create_dir(dst.join("posts"))
        .context("Could not create the `posts` directory")?;

    fs::create_dir(dst.join("tags"))
        .context("Could not create the `tags` directory")?;

    Ok(())
}

fn build_site(src: &Path, dst: &Path) -> Result<()> {
    let tera = Tera::new(
        &format!("{}/theme/templates/*.html", src.to_string_lossy()),
    )?;

    let mut tags: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();

    for post in find_posts(src)? {
        let post = post?;

        println!(" -  Processing post `{}`", post.file_stem().unwrap().to_string_lossy());

        let post = load_post(&post)
            .context("Could not load post")?;

        for tag in &post.tags {
            tags.entry(tag.to_owned())
                .or_default()
                .insert(post.id.clone());
        }

        let page_file = dst
            .join("posts")
            .join(&post.id)
            .with_extension("html");

        let page_content = render_post_page(&tera, &post)
            .context("Could not render post page")?;

        fs::write(page_file, page_content)
            .context("Could not save post page")?;
    }

    let tags: Vec<_> = tags
        .into_iter()
        .map(|(id, posts)| Tag { id, posts })
        .collect();

    for tag in &tags {
        println!(" -  Processing tag `{:?}`", tag.id);

        let page_file = dst
            .join("tags")
            .join(&tag.id)
            .with_extension("html");

        let page_content = render_tag_page(&tera, tag)
            .context("Could not render tag page")?;

        fs::write(page_file, page_content)
            .context("Could not save tag page")?;
    }

    println!(" -  Processing tags");

    Ok(())
}

fn find_posts(src: &Path) -> Result<impl Iterator<Item=Result<PathBuf>>> {
    let posts = src.join("posts");

    if !posts.exists() {
        bail!("Could not find `posts` directory (tried: {})", posts.to_string_lossy());
    }

    let posts = glob::glob(&format!("{}/*.adoc", posts.to_string_lossy()))
        .with_context(|| format!("Could not enumerate posts at `{}`", posts.to_string_lossy()))?;

    let posts = posts.map(|post| post.map_err(Into::into));

    Ok(posts)
}

fn load_post(post: &Path) -> Result<Post> {
    let id = post
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .into();

    let content = fs::read_to_string(post)?;

    let mut cmd = Command::new("asciidoctor")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("Could not spawn `asciidoctor`")?;

    let pipe = cmd.stdin
        .as_mut()
        .ok_or_else(|| anyhow!("Could not attach to `asciidoctor`'s stdin"))?;

    pipe.write(content.as_bytes())
        .context("Could not write to `asciidoctor`'s stdin")?;

    let cmd = cmd
        .wait_with_output()
        .context("Could not wait for `asciidoctor`")?;

    if !cmd.status.success() {
        bail!("{}", String::from_utf8_lossy(&cmd.stderr));
    }

    let content = String::from_utf8_lossy(&cmd.stdout)
        .into();

    Ok(Post {
        id,
        title: "".to_string(),
        summary: "".to_string(),
        content,
        tags: Default::default(),
    })
}

fn render_post_page(tera: &Tera, post: &Post) -> Result<String> {
    let mut ctxt = tera::Context::new();

    ctxt.insert("post", post);

    let page = tera
        .render("post.html", &ctxt)
        .context("Could not render page")?;

    html_minifier::minify(page)
        .map_err(|err| anyhow!("{}", err))
        .context("Could not minify page")
}

fn render_tag_page(tera: &Tera, tag: &Tag) -> Result<String> {
    let mut ctxt = tera::Context::new();

    ctxt.insert("tag", tag);

    let page = tera
        .render("tag.html", &ctxt)
        .context("Could not render page")?;

    html_minifier::minify(page)
        .map_err(|err| anyhow!("{}", err))
        .context("Could not minify page")
}

fn build_theme(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    println!(" -  Processing `style.scss`");

    let cmd = Command::new("sass")
        .arg("-C")
        .arg("--sourcemap=none")
        .arg(src.join("theme").join("style.scss"))
        .arg(dst.join("assets").join("style.css"))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Could not spawn `sass`")?;

    let cmd = cmd
        .wait_with_output()
        .context("Could not wait for `sass`")?;

    if !cmd.status.success() {
        bail!("{}", String::from_utf8_lossy(&cmd.stderr));
    }

    Ok(())
}