use std::error::Error;

use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;
use env_file_reader::read_file;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;

struct MyHandler;

#[serenity::async_trait]
impl EventHandler for MyHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        // documentation uses if (i prefer match)
        match msg.content.as_str() {
            "!hello" => {
                let _ = msg.channel_id.say(&ctx, "Hello, Discord!");
            },
            _ => ()
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Grab variable from `.env` file
    // https://docs.rs/env-file-reader/latest/env_file_reader/#usage
    let env_variables = read_file(".env")?;

// default configuration is to join chat as anonymous.
    let config = ClientConfig::default();
    let (mut incoming_messages, twitch_client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    // join a channel
    // This function only returns an error if the passed channel login name is malformed,
    // so in this simple case where the channel name is hardcoded we can ignore the potential
    // error with `unwrap`.
    twitch_client.join("sodapoppin".to_owned()).unwrap();


    // Add "DISCORD_TOKEN" to the token variable
    let token = &env_variables["DISCORD_TOKEN"];

    // Create and start the client with "MyHandler"
    // https://docs.rs/serenity/0.12.1/serenity/client/struct.Client.html#examples
    let mut discord_client = Client::builder(token, GatewayIntents::default()).event_handler(MyHandler).await?;

// first thing you should do: start consuming incoming messages,
    // otherwise they will back up.
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            println!("Received message: {:?}", message);
        }
    });



    discord_client.start().await?;


    // keep the tokio executor alive.
    // If you return instead of waiting the background task will exit.
    join_handle.await.unwrap();

    Ok(())
}
