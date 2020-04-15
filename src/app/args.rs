use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
    pub src: PathBuf,
    pub dst: PathBuf,

    #[structopt(short, long)]
    pub watch: bool,
}
