use std::path::PathBuf;

use stdout_compiler::input::{PostsRepository, TemplatesRepository};
use stdout_compiler::output::{SiteCompiler, ThemeCompiler};

pub struct Context {
    pub src: PathBuf,
    pub posts: PostsRepository,
    pub templates: TemplatesRepository,
    pub theme: ThemeCompiler,
    pub site: SiteCompiler,
}
