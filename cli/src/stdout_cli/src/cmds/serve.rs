use std::net::SocketAddr;
use std::path::PathBuf;

use anyhow::*;

use stdout_server::Server;

pub async fn serve(dst: PathBuf, addr: SocketAddr) -> Result<()> {
    println!("[+] Serving");
    println!(" -  dst: {}", dst.to_string_lossy());
    println!(" -  addr: {}", addr);

    Server::new(dst, addr)
        .start()
        .await?;

    Ok(())
}