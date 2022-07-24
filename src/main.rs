mod commands;
mod event;

use serenity::Client;
use commands::get_framework;
use event::Events;

#[tokio::main]
async fn main() -> Result<(), serenity::Error> {
    let mut client = Client::builder("OTU0NzY4NTM1ODYxNzQ3NzIy.GEMk7k.LTRwtY7bct-DzCP2pUdfrJycIX2ZYw_NDgwOoI")
        .framework(get_framework())
        .event_handler(Events)
        .await?;

    client.start().await?;

    Ok(())
}
