use anyhow::*;
use tokio::runtime;

pub use self::{
    cmd::*,
    cmds::*,
};

mod cmd;
mod cmds;

#[paw::main]
fn main(cmd: Command) -> Result<()> {
    let mut runtime = runtime::Builder::new()
        .enable_all()
        .basic_scheduler()
        .build()?;

    runtime.block_on(async move {
        match cmd {
            Command::Build { src, dst, watch, release: _ } => {
                cmds::build(src, dst, watch)
                    .await
            }

            Command::Serve { dst, addr } => {
                cmds::serve(dst, addr)
                    .await
            }
        }
    })
}
