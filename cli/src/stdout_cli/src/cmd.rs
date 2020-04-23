use std::net::SocketAddr;
use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    Build {
        src: PathBuf,
        dst: PathBuf,

        #[structopt(short, long)]
        release: bool, // @todo if enabled, check outgoing links, self-references etc.

        #[structopt(short, long)]
        watch: bool,
    },

    Serve {
        dst: PathBuf,
        addr: SocketAddr,
    },
}
