use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{bail, Context as _, Result};
use hyper::Body;
use tokio::fs;
use tokio_util::codec::{BytesCodec, FramedRead};
use warp::{Filter, path, reject, reply};

use self::context::*;

mod context;

pub struct Server {
    dir: PathBuf,
    addr: SocketAddr,
}

impl Server {
    pub fn new(dir: PathBuf, addr: SocketAddr) -> Self {
        Self { dir, addr }
    }

    pub async fn start(self) -> Result<()> {
        if fs::metadata(&self.dir).await.is_err() {
            bail!("Could not open the site's directory");
        }

        let ctxt = Arc::new(Context {
            dir: self.dir,
        });

        let ctxt = move || Arc::clone(&ctxt);

        let catch_all = warp::any()
            .map(ctxt.clone())
            .and_then(show_404);

        let show_home = warp::get()
            .map(ctxt.clone())
            .and(path::end())
            .and_then(show_home);

        let show_post = warp::get()
            .map(ctxt.clone())
            .and(path::path("posts"))
            .and(path::param())
            .and(path::end())
            .and_then(show_post);

        let show_tags = warp::get()
            .map(ctxt.clone())
            .and(path::path("tags"))
            .and_then(show_tags);

        let show_tag = warp::get()
            .map(ctxt.clone())
            .and(path::path("tags"))
            .and(path::param())
            .and(path::end())
            .and_then(show_tag);

        let router = show_home
            .or(show_post)
            .or(show_tags)
            .or(show_tag)
            .or(catch_all);

        warp::serve(router)
            .run(self.addr)
            .await;

        Ok(())
    }
}

async fn show_404(ctxt: Arc<Context>) -> Result<impl warp::Reply, reject::Rejection> {
    let file = ctxt.dir.join("404.html");

    stream_file(&file)
        .await
        .map(reply::html)
        .map_err(|_| todo!())
}

async fn show_home(ctxt: Arc<Context>) -> Result<impl warp::Reply, reject::Rejection> {
    let file = ctxt.dir.join("home.html");

    stream_file(&file)
        .await
        .map(reply::html)
        .map_err(|_| todo!())
}

async fn show_post(ctxt: Arc<Context>, id: String) -> Result<impl warp::Reply, reject::Rejection> {
    let file = ctxt
        .dir
        .join("posts")
        .join(id)
        .with_extension("html");

    stream_file(&file)
        .await
        .map(reply::html)
        .map_err(|_| reject::reject())
}

async fn show_tags(ctxt: Arc<Context>) -> Result<impl warp::Reply, reject::Rejection> {
    let file = ctxt.dir.join("tags.html");

    stream_file(&file)
        .await
        .map(reply::html)
        .map_err(|_| todo!())
}

async fn show_tag(ctxt: Arc<Context>, id: String) -> Result<impl warp::Reply, reject::Rejection> {
    let file = ctxt
        .dir
        .join("tags")
        .join(id)
        .with_extension("html");

    stream_file(&file)
        .await
        .map(reply::html)
        .map_err(|_| reject::reject())
}

async fn stream_file(file: &Path) -> Result<Body> {
    let file = fs::File::open(file)
        .await
        .with_context(|| format!("Could not read file: {}", file.to_string_lossy()))?;

    let file = FramedRead::new(file, BytesCodec::new());

    Ok(hyper::Body::wrap_stream(file))
}