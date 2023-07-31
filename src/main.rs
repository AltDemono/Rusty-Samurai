#[macro_use]
extern crate simple_log;

use simple_log::LogConfigBuilder;

rust_i18n::i18n!("locales");

use teloxide::Bot;

mod bot;
pub use bot::*;

mod actions;
mod utils;

// const TOKEN_KEY: &str = "<KEY>";
const TOKEN_KEY: &str = "5980865809:AAEUeekxtY15UmRBjUzKV5g3I_9KlPhUfvM";

// fill this field with your reports_chat_id
const REPORTS_CHAT_ID: i64 = -1001778994193;

/*
    Fill this with your Telegram user token. NOT USED YET!
*/
const OWNER_ID: u64 = 1227082481;

const SHOW_COMMANDS: bool = true;

/* 
    List of block times in seconds for which the bot should not send messages to the user. 
    Change this if you want the bot to send messages to the user more often. 
*/
const BLOCK_TIMES: [i64; 9] = [300, 600, 1800, 3600, 7200, 14400, 28800, 86400, 604800];

pub type TelegramBot = teloxide::adaptors::DefaultParseMode<Bot>;

// Main function
#[tokio::main]
async fn main() {
    let config = LogConfigBuilder::builder()
        .path("./log/bot.log")
        .size(1 * 100)
        .roll_count(10)
        .time_format("%Y-%m-%d %H:%M:%S.%f") //E.g:%H:%M:%S.%f
        .level("debug")
        .output_file()
        .output_console()
        .build();

    simple_log::new(config).unwrap();

    // Default locale when bot start
    rust_i18n::set_locale("en-us");
    info!("Starting a rusty samurai bot. \nTelegram token is {}", TOKEN_KEY);

    bot::bot_init().await;
}