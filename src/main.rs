#![feature(try_blocks)]

use std::net::SocketAddr;
use std::path::PathBuf;

use anyhow::*;
use structopt::StructOpt;

mod backend;
mod frontend;
mod middleend;

#[derive(Debug, StructOpt)]
enum Command {
    Check {
        #[structopt(short, long, default_value = "site")]
        src: PathBuf,
    },

    Serve {
        #[structopt(short, long, default_value = "site")]
        src: PathBuf,

        #[structopt(short, long, default_value = "127.0.0.1:1337")]
        addr: SocketAddr,
    },
}

#[paw::main]
#[tokio::main]
async fn main(cmd: Command) {
    let result: Result<()> = try {
        match cmd {
            Command::Check { src } => {
                println!("Building");

                backend::build(src).await?;

                println!("All ok!");
            }

            Command::Serve { src, addr } => {
                println!("Building");

                let site = backend::build(src).await?;

                println!("Serving");

                frontend::serve(site, addr).await?;
            }
        }
    };

    // @todo
    result.unwrap();
}
