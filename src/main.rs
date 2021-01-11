use music_gen::*;
use music_gen::theory::*;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

#[group]
#[commands(alive, help, notes)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let args: Vec<String> = std::env::args().collect();
    let mut client = Client::builder(&args[1])
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

fn break_up(msg: String) -> Vec<String>{
    if msg.len() <= 1000 { vec![msg] }
    else {
        let mut res = Vec::new();
        let mut string = String::new();
        let mut l = 0;
        for (i, c) in msg.chars().enumerate(){
            string.push(c);
            if i - l >= 1000 && c == '\n' {
                res.push(string);
                string = String::new();
                l = i;
            }
        }
        res.push(string);
        res
    }
}

#[command]
async fn alive(ctx: &Context, msg: &Message) -> CommandResult{
    msg.reply(ctx, &"I   A M   A L I V E").await?;
    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult{
    let empleh = &"Usage:\n\t~notes <comma separated notes>\n\t\texamples:\n\t\t\t~notes a,c,e,g\n\t\t\t~notes As,Db,Fb,g";
    msg.reply(ctx, empleh).await?;
    Ok(())
}

#[command]
async fn notes(ctx: &Context, msg: &Message) -> CommandResult{
    let prepared = msg.content.clone().split(' ').into_iter().skip(1).collect::<String>();
    println!("{}", prepared);
    let test_outp = notes_analysis(prepared, ChordStyling::Std);
    let broken = break_up(test_outp);
    for piece in broken{
        msg.reply(ctx, &piece).await?;
    }
    Ok(())
}
