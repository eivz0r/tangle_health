use iota_streams::app_channels::api::tangle::{Author, ChannelType};
use iota_streams::app::transport::tangle::client::Client;
use anyhow::Result;

#[tokio::main]
pub async fn create_channel(seed: &String) -> Result<String> {
    let node = "https://api.lb-0.h.chrysalis-devnet.iota.cafe/";
    let client = Client::new_from_url(node);

    let mut author = Author::new(&seed, ChannelType::SingleBranch, client);

    let ann_address = author.send_announce().await?;   

    println!("{}", ann_address.to_string());

    Ok(ann_address.to_string())
}