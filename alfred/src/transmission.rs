use std::env;

use transmission_rpc::types::{BasicAuth, Result, RpcResponse};
use transmission_rpc::types::{Id, Torrent, TorrentGetField, Torrents};
use transmission_rpc::types::{TorrentAddArgs, TorrentAdded};
use transmission_rpc::TransClient;

// #[tokio::main]
// async fn main() -> Result<()> {

pub struct Transmission {
    client: TransClient,
}

impl Transmission {
    pub fn new() -> Self {
        let url = env::var("TURL").unwrap();
        Self {
            client: TransClient::new(&url),
        }
    }
    pub async fn torrent_add(&self) -> Result<()> {
        println!("asdasd");
        let add: TorrentAddArgs = TorrentAddArgs {
            filename: Some(
                "https://releases.ubuntu.com/21.10/ubuntu-21.10-desktop-amd64.iso.torrent"
                    .to_string(),
            ),
            ..TorrentAddArgs::default()
        };
        let res: RpcResponse<TorrentAdded> = self.client.torrent_add(add).await?;
        println!("Add result: {:?}", &res.is_ok());
        println!("response: {:?}", &res);

        Ok(())
    }

    async fn torrent_get(&self) -> Result<()> {
        let res: RpcResponse<Torrents<Torrent>> = self.client.torrent_get(None, None).await?;
        let names: Vec<&String> = res
            .arguments
            .torrents
            .iter()
            .map(|it| it.name.as_ref().unwrap())
            .collect();
        println!("{:#?}", names);

        let res1: RpcResponse<Torrents<Torrent>> = self
            .client
            .torrent_get(
                Some(vec![TorrentGetField::Id, TorrentGetField::Name]),
                Some(vec![Id::Id(1), Id::Id(2), Id::Id(3)]),
            )
            .await?;
        let first_three: Vec<String> = res1
            .arguments
            .torrents
            .iter()
            .map(|it| {
                format!(
                    "{}. {}",
                    &it.id.as_ref().unwrap(),
                    &it.name.as_ref().unwrap()
                )
            })
            .collect();
        println!("{:#?}", first_three);

        let res2: RpcResponse<Torrents<Torrent>> = self
            .client
            .torrent_get(
                Some(vec![
                    TorrentGetField::Id,
                    TorrentGetField::HashString,
                    TorrentGetField::Name,
                ]),
                Some(vec![Id::Hash(String::from(
                    "64b0d9a53ac9cd1002dad1e15522feddb00152fe",
                ))]),
            )
            .await?;
        let info: Vec<String> = res2
            .arguments
            .torrents
            .iter()
            .map(|it| {
                format!(
                    "{:5}. {:^45} {}",
                    &it.id.as_ref().unwrap(),
                    &it.hash_string.as_ref().unwrap(),
                    &it.name.as_ref().unwrap()
                )
            })
            .collect();
        println!("{:#?}", info);

        Ok(())
    }
}

// [tokio::main]
// async fn main() -> Result<()> {
