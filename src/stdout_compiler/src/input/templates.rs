use anyhow::*;
use html_minifier::minify;
use tera::{Context as TeraContext, Tera};

pub use self::repository::*;

mod repository;

pub struct Templates {
    tera: Tera,
}

impl Templates {
    pub fn new(tera: Tera) -> Self {
        Self { tera }
    }

    pub fn render(&self, template: &str, build_ctxt: impl FnOnce(&mut TeraContext)) -> Result<String> {
        let mut ctxt = TeraContext::new();

        build_ctxt(&mut ctxt);

        let out = self.tera
            .render(template, &ctxt)
            .context("Could not render template")?;

        minify(&out)
            .map_err(|err| anyhow!("{}", err))
            .context("Could not minify output")
    }
}
