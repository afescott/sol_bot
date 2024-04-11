use serenity::{
    all::{ChannelId, GatewayIntents, Message},
    builder::GetMessages,
    client::{Context, EventHandler},
    http, Client,
};
use tokio::sync::mpsc::Sender;

use crate::{api::filter::filter_msg, repositories::models::Token};

const APP_ID: &str = "1225649067506532515";

const PUB_KEY: &str = "df17ba54623654893e5e094695b24e2027916f9c4c900dd5155ee64d1255149a";

const TOKEN: &str = "Bot MTIyNTY0OTA2NzUwNjUzMjUxNQ.GTgTQL.3pwOAa9MYHmwIs_hF5L2FOnd6sngNbxKTtcBms";

struct Handler {
    tokens: Vec<Token>,
    tx: Sender<Vec<Token>>,
}

#[async_trait::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        /*         let asfaf = serde_json::to_value(msg).unwrap(); */
        /*         println!("{:?}", msg); */

        let res = filter_msg(msg).await;

        if let Err(err) = self.tx.send(res).await {
            println!("Channel send error: {:?}", err);
        }
    }
}

pub async fn webhook_messages(tx: Sender<Vec<Token>>) {
    let channel_id = ChannelId::new(1225593082427342902);
    let client = http::Http::new(TOKEN);

    let builder = GetMessages::new().limit(25);
    let _messages = channel_id.messages(&client, builder).await.unwrap();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will automatically prepend
    // your bot token with "Bot ", which is a requirement by Discord for bot users.
    let mut client = Client::builder(TOKEN, intents)
        .event_handler(Handler {
            tokens: Vec::new(),
            tx,
        })
        .await
        .expect("Err creating client");

    client.start().await;
}
