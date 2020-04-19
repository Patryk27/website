use std::path::PathBuf;

use anyhow::*;
use tokio::runtime;
use warp::Filter;

pub struct Server {
    dir: PathBuf,
}

impl Server {
    pub fn new(dir: PathBuf) -> Self {
        Self { dir }
    }

    pub fn start(self) -> Result<()> {
        let routes = warp::any()
            .map(|| "Hi!");

        let server = warp::serve(routes)
            .run(([127, 0, 0, 1], 1337));

        let mut runtime = runtime::Builder::new()
            .enable_all()
            .basic_scheduler()
            .build()?;

        runtime.block_on(server);

        Ok(())
    }
}