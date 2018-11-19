//! # 志鳥/Sitri
//! 志鳥/SitriはSCP-JP Discordサーバーで稼働する用のBOTです
//! 現在構築中

#[macro_use]
extern crate serenity;
extern crate chrono;
extern crate cute_dnd_dice;
extern crate regex;

use serenity::client::{Client, Context, EventHandler};
use serenity::framework::StandardFramework;
use serenity::model::gateway::Ready;
use std::env;

mod commands;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is ready.", &ready.user.name);
    }
}

fn main() {
    let token = &env::var("DISCORD_TOKEN").expect("no envvar DISCORD_TOKEN");
    let mut client = Client::new(token, Handler).expect("constracting new client error");

    // なぜかここでチェーン繋げないと借用エラーになる
    let framework = StandardFramework::new()
        .configure(|conf| conf.prefix(">>")) // ">>command" でコマンド発火
        .group("test", |grp| grp.cmd("ping", commands::test::ping))
        .group("system", |grp| grp.cmd("quit", commands::system::quit))
        .group("scp", |grp| {
            grp.cmd("page", commands::scp::page)
                .cmd("sandbox", commands::scp::sandbox)
                .cmd("draft", commands::scp::draft)
                .cmd("dice", commands::scp::dice)
        });

    client.with_framework(framework);

    match client.start() {
        Ok(_) => println!("Sitri Awakening",),
        Err(e) => println!("connection error: {:?}", e),
    }
}
