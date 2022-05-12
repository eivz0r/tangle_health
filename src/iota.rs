use iota_streams::app_channels::api::tangle::{ Address, Author, Subscriber, ChannelType};
use iota_streams::app::transport::tangle::client::Client;
use anyhow::Result;
use std::str::FromStr;

#[tokio::main]
pub async fn create_channel(seed: String) -> Result<String> {
    let node = "https://api.lb-0.h.chrysalis-devnet.iota.cafe/";
    let client = Client::new_from_url(node);

    let mut author = Author::new(&seed, ChannelType::SingleBranch, client);

    let ann_address = author.send_announce().await?;

    Ok(ann_address.to_string())
}

#[tokio::main]
pub async fn subscribe_to_channel(seed: String, ann_address: String) -> Result<String> {
    let node = "https://api.lb-0.h.chrysalis-devnet.iota.cafe/";
    let client = Client::new_from_url(node);

    let mut subscriber = Subscriber::new(&seed, client);
    
    // Create Address object from announcement address string
    let ann_address = Address::from_str(&ann_address.to_string())?;   
    //println!("{}", ann_address.to_string());

    // Process the announcement message
    subscriber.receive_announcement(&ann_address).await?;


    Ok("Success".to_owned())
}