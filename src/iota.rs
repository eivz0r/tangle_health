use iota_streams::app_channels::api::tangle::{ Address, Author, Bytes, Subscriber, ChannelType, MessageContent };
use iota_streams::app::transport::tangle::client::Client;
use anyhow::Result;
use core::str::FromStr;

#[tokio::main]
pub async fn create_channel(author_seed: String, client: Client) -> Result<String> {
    let mut author = Author::new(&author_seed, ChannelType::SingleBranch, client);

    let ann_link = author.send_announce().await?;

    Ok(ann_link.to_string())
}

#[tokio::main]
pub async fn subscribe_to_channel(sub_seed: String, ann_link: String, client: Client) -> Result<String> {
    let mut subscriber = Subscriber::new(&sub_seed, client);

    let ann_link_adr = Address::from_str(&ann_link)?;

    subscriber.receive_announcement(&ann_link_adr).await?;

    let sub_link = subscriber.send_subscribe(&ann_link_adr).await?;

    Ok(sub_link.to_string())
}

#[tokio::main]
pub async fn process_subscription(author_seed: String, ann_link: String, sub_link: String, client: Client) -> Result<String> {
    let mut author = Author::new(&author_seed, ChannelType::SingleBranch, client);

    let ann_link_adr = Address::from_str(&ann_link)?;

    let sub_link_adr = Address::from_str(&sub_link)?;
    
    author.receive_subscribe(&sub_link_adr).await?;

    let (keyload_link, _seq) = author.send_keyload_for_everyone(&ann_link_adr).await?;

    Ok(keyload_link.to_string())
}

#[tokio::main]
pub async fn send_health_data(author_seed: String, keyload_link: String, client: Client) -> Result<String> {
    let mut author = Author::new(&author_seed, ChannelType::SingleBranch, client.clone());

    let msg_inputs = vec![
        "HR:  80  -  Steps:  2000  -  Calories:  1765", "HR:  78  -  Steps:  2111  -  Calories:  1803", 
        "HR:  74  -  Steps:  2245  -  Calories:  1876", "HR:  85  -  Steps:  2302  -  Calories:  1920",
        "HR:  72  -  Steps:  2340  -  Calories:  1997", "HR:  86  -  Steps:  2360  -  Calories:  2002",
        "HR:  74  -  Steps:  2400  -  Calories:  2103", "HR:  67  -  Steps:  2410  -  Calories:  1920",
    ];

    let mut prev_msg_link = Address::from_str(&keyload_link)?;

    for input in &msg_inputs {
        let (msg_link, _seq_link) = author.send_signed_packet(
            &prev_msg_link,
            &Bytes::default(),
            &Bytes(input.as_bytes().to_vec()),
        ).await?;
        println!("Sent msg: {}, tangle index: {:#}", msg_link, msg_link.to_msg_index());
        prev_msg_link = msg_link;
    }
    
    Ok("Health Data Sent".to_string())
}

#[tokio::main]
pub async fn fetch_health_data(sub_seed: String, client: Client) -> Result<Vec<String>> {

    let mut subscriber = Subscriber::new(&sub_seed, client);

    let retrieved_msgs = subscriber.fetch_next_msgs().await;

    let messages = retrieved_msgs.unwrap_or_default();

    let processed_msgs = messages
        .iter()
        .map(|msg| {
            let content = &msg.body;
            match content {
                MessageContent::SignedPacket {
                    pk: _,
                    public_payload: _,
                    masked_payload,
                } => String::from_utf8(masked_payload.0.to_vec()).unwrap(),
                _ => String::default(),
            }
        })
        .filter(|s| s != &String::default())
        .collect::<Vec<String>>();
    
    Ok(processed_msgs)
}

// Run all IOTA Streams functions
#[tokio::main]
pub async fn run_all(seed: String) -> Result<Vec<String>> {
    
    // ----------------------- Create Author instance from unique seed -----------------------
    let node = "https://api.lb-0.h.chrysalis-devnet.iota.cafe/";
    let client = Client::new_from_url(node);
    let mut author = Author::new(&seed, ChannelType::SingleBranch, client.clone());

    // Send announcement of newly created channel, returns announcement link
    let ann_link = author.send_announce().await?;

    // ----------------------- Create Subscriber instance from unique seed -----------------------
    let mut subscriber = Subscriber::new("SubscriberA433", client);

    subscriber.receive_announcement(&ann_link).await?;

    let sub_address = subscriber.send_subscribe(&ann_link).await?;
    
    // ----------------------- Author processes the subscription -----------------------
    author.receive_subscribe(&sub_address).await?;

    let (keyload_link, _seq) = author.send_keyload_for_everyone(&ann_link).await?;

    // ----------------------- Author sends health data as a vector of strings
    let msg_inputs = vec![
        "HR:  80  -  Steps:  2000  -  Calories:  1765", "HR:  78  -  Steps:  2111  -  Calories:  1803", 
        "HR:  74  -  Steps:  2245  -  Calories:  1876", "HR:  85  -  Steps:  2302  -  Calories:  1920",
        "HR:  72  -  Steps:  2340  -  Calories:  1997", "HR:  86  -  Steps:  2360  -  Calories:  2002",
        "HR:  74  -  Steps:  2400  -  Calories:  2103", "HR:  67  -  Steps:  2410  -  Calories:  1920",
    ];

    let mut prev_msg_link = keyload_link;

    for input in &msg_inputs {
        let (msg_link, _seq_link) = author.send_signed_packet(
            &prev_msg_link,
            &Bytes::default(),
            &Bytes(input.as_bytes().to_vec()),
        ).await?;
        println!("Sent msg: {}, tangle index: {:#}", msg_link, msg_link.to_msg_index());
        prev_msg_link = msg_link;
    }

    // ----------------------- Subscriber fetches health data sent by the Author -----------------------
    let retrieved_msgs = subscriber.fetch_next_msgs().await;

    let messages = retrieved_msgs.unwrap_or_default();

    let processed_msgs = messages
        .iter()
        .map(|msg| {
            let content = &msg.body;
            match content {
                MessageContent::SignedPacket {
                    pk: _,
                    public_payload: _,
                    masked_payload,
                } => String::from_utf8(masked_payload.0.to_vec()).unwrap(),
                _ => String::default(),
            }
        })
        .filter(|s| s != &String::default())
        .collect::<Vec<String>>();

    Ok(processed_msgs)
}