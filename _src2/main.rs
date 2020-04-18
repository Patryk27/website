use std::collections::BTreeMap;
use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
    pub src: PathBuf,
    pub dst: PathBuf,

    #[structopt(short, long)]
    pub watch: bool,
}

mod input;
mod output;

#[paw::main]
fn main(args: Args) {
    let posts = BTreeMap::new();
    let tags = BTreeMap::new();

    for post in input::posts::find(&args.src) {
        let post = input::posts::load(&args.src, &post)?;
    }

    let templates = input::templates::load(&args.src);

    input::theme::build(&args.src, &args.dst)?;
}
