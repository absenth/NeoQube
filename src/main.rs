use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();

    // default configuration is to join chat as anonymous.
    // FIXME: configure this to use the botsenth545 user
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    // first thing to do: start consuming incoming messages,
    // otherwise they will back up.
    let join_handle = tokio::spawn(async move {
      while let Some(message) = incoming_messages.recv().await {
            tracing::info!("Received message: {:?}", message);
      }
    });

    // join a channel
    // This function only returns an error if the passed channel login name is malformed,
    // So in this simple case where the channel name is hardcoded we can ignore the potential
    // error with `unwrap`.
    client.join("absenth762".to_owned()).unwrap();

    // keep the tokio executor alive.
    // If you return instead of waiting the background task will exit.
    join_handle.await.unwrap();
}

