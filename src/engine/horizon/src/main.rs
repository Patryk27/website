#![feature(try_blocks)]

mod render_feed;
mod render_post;
mod utils;

use crate::utils::Env;
use anyhow::Result;
use clap::Parser;
use std::io;

#[derive(Clone, Debug, Parser)]
enum Cmd {
    RenderFeed(#[clap(subcommand)] render_feed::Cmd),
    RenderPost(#[clap(subcommand)] render_post::Cmd),
}

fn main() -> Result<()> {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut stderr = io::stderr().lock();

    let mut env = Env {
        stdin: &mut stdin,
        stdout: &mut stdout,
        stderr: &mut stderr,
    };

    match Cmd::parse() {
        Cmd::RenderFeed(cmd) => cmd.run(&mut env),
        Cmd::RenderPost(cmd) => cmd.run(&mut env),
    }
}
