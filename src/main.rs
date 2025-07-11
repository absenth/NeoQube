const CHANNELS: &[&str] = &["#absenth762"];

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = tmi::Client::anonymous().await?;
    client.join_all(CHANNELS).await?;

    loop {
        let msg = client.recv().await?;
        match msg.as_typed()? {
            tmi::Message::Privmsg(msg) => {
                println!("{}: {}", msg.sender().name(), msg.text());
            }
            tmi::Message::Reconnect => {
                client.reconnect().await?;
                client.join_all(CHANNELS).await?;
            }
            tmi::Message::Ping(ping) => {
                client.pong(&ping).await?;
            }
            _ => {}
        }
    }
}
