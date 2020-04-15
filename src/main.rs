#![feature(try_blocks)]

use std::path::PathBuf;

use anyhow::*;
use structopt::StructOpt;

mod cmds;
mod models;

#[derive(Debug, StructOpt)]
enum Command {
    Build {
        src: PathBuf,
        dst: PathBuf,
    },

    Watch {
        src: PathBuf,
        dst: PathBuf,
    },
}

#[paw::main]
fn main(cmd: Command) {
    let result = match cmd {
        Command::Build { src, dst } => {
            cmds::build(src, dst)
        }

        Command::Watch { src, dst } => {
            cmds::watch(src, dst)
        }
    };

    // @todo
    result.unwrap();
}
