use anyhow::*;

pub use self::{
    cmd::*,
    cmds::*,
};

mod cmd;
mod cmds;

#[paw::main]
fn main(cmd: Command) -> Result<()> {
    match cmd {
        Command::Build { src, dst, watch, release: _ } => {
            cmds::build(src, dst, watch)
        }
    }
}
