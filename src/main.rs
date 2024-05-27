use std::env;
use host::Host;
use log::info;

use crate::client_handler::Session;

mod host;
mod client_handler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let bastion = Host::new(
        &env::var("FIRST_JUMP_HOSTNAME").unwrap(),
        env::var("FIRST_JUMP_PORT").unwrap().parse().unwrap(),
        &env::var("FIRST_JUMP_USERNAME").unwrap(),
        &env::var("FIRST_JUMP_PASSWORD").unwrap(),
    );

    info!("Connecting to {:?}:{:?} by {:?}", bastion.hostname, bastion.port, bastion.username);
    let mut ssh = Session::connect(bastion).await.unwrap();
    info!("Connected");

    let code = ssh.call("ls -l").await?;
    info!("Exit code: {}", code);

    ssh.close().await?;
    Ok(())
}
