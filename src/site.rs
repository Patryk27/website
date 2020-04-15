use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::*;
use chrono::Utc;

use crate::{Post, Tag};

pub struct Site {
    dir: PathBuf,
    posts: HashMap<String, Post>,
    tags: HashMap<String, Tag>,
    dirty_posts: HashSet<String>,
    dirty_tags: HashSet<String>,
}

impl Site {
    pub fn new(dir: &Path) -> Result<Self> {
        if dir.exists() {
            bail!("Destination directory already exists, aborting");
        }

        fs::create_dir(&dir)
            .context("Could not create the main directory")?;

        fs::write(dir.join(".gitignore"), "*")
            .context("Could not create `.gitignore`")?;

        fs::write(dir.join(".timestamp"), Utc::now().to_rfc2822())
            .context("Could not create `.timestamp`")?;

        fs::create_dir(dir.join("assets"))
            .context("Could not create the `assets` directory")?;

        fs::create_dir(dir.join("posts"))
            .context("Could not create the `posts` directory")?;

        fs::create_dir(dir.join("tags"))
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
            // self.tags
            //     .entry(tag.to_owned())
            //     .or_default()
            //     .insert(post.id.clone());

            self.dirty_tags
                .insert(tag.to_owned());
        }

        self.dirty_posts.insert(post.id.clone());
        self.posts.insert(post.id.clone(), post);
    }

    pub fn del_post(&mut self, id: String) {
        for (_, tag) in &mut self.tags {
            tag.posts.remove(&id);
        }

        self.posts.remove(&id);
        self.dirty_posts.insert(id);
    }

    pub fn render(&mut self) -> Result<()> {
        Ok(())

        // let mut ctxt = tera::Context::new();
        //
        // ctxt.insert("post", post);
        //
        // let page = tera
        //     .render("post.html", &ctxt)
        //     .context("Could not render page")?;
        //
        // html_minifier::minify(page)
        //     .map_err(|err| anyhow!("{}", err))
        //     .context("Could not minify page")
    }

    // fn xx_flush(&mut self) -> Result<()> {
    //     let dirty_posts: Vec<_> = self.dirty_posts
    //         .drain()
    //         .collect();
    //
    //     for dirty_post in dirty_posts {
    //         self.flush_post(&dirty_post)
    //             .with_context(|| format!("Could not flush post `{}`", dirty_post))?;
    //     }
    //
    //     Ok(())
    // }
    //
    // fn xx_flush_post(&mut self, id: &str) -> Result<()> {
    //     let id = &self.posts[id];
    //
    //     let page_file = self
    //         .dst
    //         .join("posts")
    //         .join(&id)
    //         .with_extension("html");
    //
    //     let page_content = render_post_page(&tera, &post)
    //         .context("Could not render post page")?;
    //
    //     fs::write(page_file, page_content)
    //         .context("Could not save post page")?;
    // }
}