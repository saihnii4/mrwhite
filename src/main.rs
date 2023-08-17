use discord::model::Event;
use discord::Discord;
use std::env;

fn main() {
    let discord =
        Discord::from_bot_token(&env::var("").expect("Expected token")).expect("login failed");
    let (mut connection, _) = discord.connect().expect("connect failed");
    println!("Ready!");
    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                println!("{} says: {}", message.author.name, message.content);
                if message.content == "!test" {
                    let _ = discord.send_message(message.channel_id, "Hello World", "", false);
                } else {
                    let _ = discord.send_message(
                        message.channel_id,
                        format!("Unknown message `{}`", message.content).as_str(), // susceptible to escaping
                        "",
                        false,
                    );
                }
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break;
            }
            Err(err) => println!("Receive error: {:?}", err),
        }
    }
}
