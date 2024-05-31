use std::{sync::Arc, time::Duration};

use russh::{client, ChannelMsg, Disconnect};
use russh_keys::load_secret_key;
use tokio::io::AsyncWriteExt;

use crate::host::{HostWithKey, HostWithPassword};

pub struct ClientHandler;
impl client::Handler for ClientHandler {
    type Error = russh::Error;
}

pub struct Session {
    session: client::Handle<ClientHandler>
}
impl Session {
    pub async fn connect(
        host: HostWithPassword
    ) -> anyhow::Result<Self> {
        let config = Arc::new(client::Config {
            inactivity_timeout: Some(Duration::from_secs(60)),
            ..Default::default()
        });

        let handler = ClientHandler;

        let mut session = client::connect(config, host.addrs(), handler).await?;
        let auth = session.authenticate_password(host.username, host.password).await?;
        if !auth {
            anyhow::bail!("Authentication failed");
        }

        Ok(Self { session })
    }

    pub async fn connect_with_key(
        host: HostWithKey
    ) -> anyhow::Result<Self> {
        let config = Arc::new(client::Config {
            inactivity_timeout: Some(Duration::from_secs(60)),
            ..Default::default()
        });

        let handler = ClientHandler;

        let mut session = client::connect(config, host.addrs(), handler).await?;
        let key = load_secret_key(host.key, None)?;
        let auth = session.authenticate_publickey(host.username, Arc::new(key)).await?;
        if !auth {
            anyhow::bail!("Authentication failed");
        }

        Ok(Self { session })
    }

    pub async fn call(&mut self, command: &str) -> anyhow::Result<u32> {
        let mut channel = self.session.channel_open_session().await?;
        channel.exec(true, command).await?;

        let mut code = None;
        let mut stdout = tokio::io::stdout();

        loop {
            let Some(msg) = channel.wait().await else {
                break;
            };

            match msg {
                ChannelMsg::Data { data } => {
                    stdout.write_all(&data).await?;
                    stdout.flush().await?;
                }
                ChannelMsg::ExitStatus { exit_status } => {
                    code = Some(exit_status);
                }
                _ => {},
            }
        }

        Ok(code.expect("program did not exit successfully"))
    }

    pub async fn close(&mut self) -> anyhow::Result<()> {
        self.session.disconnect(Disconnect::ByApplication, "", "English").await?;
        Ok(())
    }
}
