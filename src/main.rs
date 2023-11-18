use std::env;
use dotenv::dotenv;

use serenity::async_trait;
use serenity::model::user::OnlineStatus;
use serenity::model::gateway::Activity;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};
use serenity::model::gateway::*;

#[group]
#[commands(ping, xd)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ : Ready) {
        ctx.set_presence(Some(Activity::watching("stuff xd")), OnlineStatus::DoNotDisturb).await;
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn xd(ctx: &Context, msg: &Message) -> CommandResult {
    println!("{} said \"{}\"", msg.author.name, msg.content);
    msg.reply(ctx, "Logged xd").await?;
    Ok(())
}
