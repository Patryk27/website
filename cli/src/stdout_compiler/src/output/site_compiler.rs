use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use anyhow::*;
use chrono::Utc;
use tokio::fs;

use crate::input::{Post, Tag, Templates};

pub struct SiteCompiler {
    dir: PathBuf,
    posts: HashMap<String, Post>,
    tags: HashMap<String, Tag>,
    dirty_posts: HashSet<String>,
    dirty_tags: HashSet<String>,
}

impl SiteCompiler {
    pub async fn new(dir: &Path) -> Result<Self> {
        if fs::metadata(dir).await.is_ok() {
            bail!("Destination directory already exists, aborting");
        }

        fs::create_dir(&dir)
            .await
            .context("Could not create the destination directory")?;

        fs::write(dir.join(".gitignore"), "*")
            .await
            .context("Could not create `.gitignore`")?;

        fs::write(dir.join(".timestamp"), Utc::now().to_rfc2822())
            .await
            .context("Could not create `.timestamp`")?;

        fs::create_dir(dir.join("assets"))
            .await
            .context("Could not create the `assets` directory")?;

        fs::create_dir(dir.join("posts"))
            .await
            .context("Could not create the `posts` directory")?;

        fs::create_dir(dir.join("tags"))
            .await
            .context("Could not create the `tags` directory")?;

        Ok(Self {
            dir: dir.into(),
            posts: Default::default(),
            tags: Default::default(),
            dirty_posts: Default::default(),
            dirty_tags: Default::default(),
        })
    }

    pub fn add_post(&mut self, post: Post) {
        self.del_post(post.id.clone());

        for tag in &post.tags {
            self.dirty_tags.insert(tag.to_owned());

            let tag = self.tags
                .entry(tag.to_owned())
                .or_insert_with(|| {
                    Tag {
                        id: tag.to_owned(),
                        posts: Default::default(),
                    }
                });

            tag.posts.insert(post.id.clone());
        }

        self.dirty_posts.insert(post.id.clone());
        self.posts.insert(post.id.clone(), post);
    }

    pub fn del_post(&mut self, id: String) {
        for (_, tag) in &mut self.tags {
            if tag.posts.remove(&id) {
                self.dirty_tags.insert(tag.id.clone());
            }
        }

        self.posts.remove(&id);
        self.dirty_posts.insert(id);
    }

    pub async fn compile(&mut self, templates: &Templates) -> Result<()> {
        self.compile_post_pages(templates)
            .await?;

        self.compile_tag_pages(templates)
            .await?;

        Ok(())
    }

    async fn compile_post_pages(&mut self, templates: &Templates) -> Result<()> {
        let posts: Vec<_> = self.dirty_posts.drain().collect();

        for post in posts {
            self.compile_post_page(templates, &post)
                .await
                .with_context(|| format!("Could not compile page for post `{}`", post))?;
        }

        Ok(())
    }

    async fn compile_post_page(&self, templates: &Templates, post: &str) -> Result<()> {
        let path = self.dir
            .join("posts")
            .join(post)
            .with_extension("html");

        if let Some(post) = self.posts.get(post) {
            let content = templates.render("post.html", |ctxt| {
                ctxt.insert("post", post)
            })?;

            fs::write(&path, content)
                .await
                .with_context(|| format!("Could not create page `{}`", path.to_string_lossy()))?;
        } else {
            fs::remove_file(&path)
                .await
                .with_context(|| format!("Could not remove page `{}`", path.to_string_lossy()))?;
        }

        Ok(())
    }

    async fn compile_tag_pages(&mut self, templates: &Templates) -> Result<()> {
        let tags: Vec<_> = self.dirty_tags.drain().collect();

        for tag in tags {
            self.compile_tag_page(templates, &tag)
                .await
                .with_context(|| format!("Could not compile page for tag `{}`", tag))?;
        }

        Ok(())
    }

    async fn compile_tag_page(&self, templates: &Templates, tag: &str) -> Result<()> {
        let path = self.dir
            .join("tags")
            .join(tag)
            .with_extension("html");

        if let Some(tag) = self.tags.get(tag) {
            let content = templates.render("tag.html", |ctxt| {
                ctxt.insert("tag", tag)
            })?;

            fs::write(&path, content)
                .await
                .with_context(|| format!("Could not create page `{}`", path.to_string_lossy()))?;
        } else {
            fs::remove_file(&path)
                .await
                .with_context(|| format!("Could not remove page `{}`", path.to_string_lossy()))?;
        }

        Ok(())
    }
}
