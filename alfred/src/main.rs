use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Sending a message can fail, due to a network error, an
    // authentication error, or lack of permissions to post in the
    // channel, so log to stdout when some error happens, with a
    // description of it.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why)
            }
        }

        if msg.content == "!how are you" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Fuck you!").await {
                println!("Error sending message: {:?}", why)
            }
        }
    }
    //
    // // Set a handler to be called on the `ready` event. This is called when a
    // // shard is booted, and a READY payload is sent by Discord. This payload
    // // contains data like the current user's guild Ids, current user data,
    // // private channels, and more.
    // //
    // // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // let framework = StandardFramework::new()
    //     .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
    //     .group(&GENERAL_GROUP);
    // let mut client = Client::builder(&token)
    //     .event_handler(Handler)
    //     .framework(framework)
    //     .await
    //     .expect("Err creating client.");
    //
    // if let Err(why) = client.start().await {
    //     println!("client err: {:?}", why)
    // }

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

// #[command]
// async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
//     msg.reply(ctx, "Pong!").await?;
//
//     Ok(())
// }
