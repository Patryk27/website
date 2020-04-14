use std::net::SocketAddr;
use std::sync::Arc;

use tokio::fs;
use warp::{Filter, path, reply};

use crate::middleend::{ArtifactId, Site};

struct Context {
    site: Site,
}

pub async fn serve(site: Site, addr: SocketAddr) -> anyhow::Result<()> {
    let ctxt = Arc::new(Context { site });

    let ctxt = warp::any()
        .map(move || Arc::clone(&ctxt));

    let post = warp::get()
        .and(ctxt.clone())
        .and(path::path("posts"))
        .and(path::param::<String>())
        .and(path::end())
        .and_then(print_post);

    warp::serve(post)
        .run(addr)
        .await;

    Ok(())
}

async fn print_post(ctxt: Arc<Context>, id: String) -> Result<impl warp::Reply, warp::Rejection> {
    // @todo log

    let post = ctxt.site.artifacts
        .get(&ArtifactId::Post { id })
        .ok_or_else(warp::reject::not_found)?;

    let post = fs::read_to_string(post)
        .await
        .map_err(|_| warp::reject::not_found())?;

    Ok(reply::html(post))
}