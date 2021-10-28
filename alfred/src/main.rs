use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;

use serenity::futures::future::Ready;
use std::env;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Sending a message can fail, due to a network error, an
    // authentication error, or lack of permissions to post in the
    // channel, so log to stdout when some error happens, with a
    // description of it.
    // async fn message(&self, ctx: Context, msg: Message) {
    //     if msg.content == "!ping" {
    //         if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
    //             println!("Error sending message: {:?}", why)
    //         }
    //     }
    // }
    //
    // // Set a handler to be called on the `ready` event. This is called when a
    // // shard is booted, and a READY payload is sent by Discord. This payload
    // // contains data like the current user's guild Ids, current user data,
    // // private channels, and more.
    // //
    // // In this case, just print what the current user's username is.
    // async fn ready(&self, _: Context, ready: Ready) {
    //     println!("{} is connected", ready.user.name);
    // }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client.");

    if let Err(why) = client.start().await {
        println!("client err: {:?}", why)
    }

    // let framework = StandardFramework::new()
    //     .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
    //     .group(&GENERAL_GROUP);
    //
    // // Login with a bot token from the environment
    // // let token = env::var("DISCORD_TOKEN").expect("token");
    // let token = "";
    // let mut client = Client::builder(token)
    //     .event_handler(Handler)
    //     .framework(framework)
    //     .await
    //     .expect("Error creating client");
    //
    // // start listening for events by starting a single shard
    // if let Err(why) = client.start().await {
    //     println!("An error occurred while running the client: {:?}", why);
    // }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
