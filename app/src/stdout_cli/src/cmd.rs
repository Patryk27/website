use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    Build {
        src: PathBuf,
        dst: PathBuf,

        #[structopt(short, long)]
        watch: bool,

        #[structopt(short, long)]
        release: bool, // @todo if enabled, check outgoing links, self-references etc.
    },

    Serve {
        dir: PathBuf,
    },
}
