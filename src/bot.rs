use teloxide::{prelude::*, utils::command::BotCommands};

use crate::{TelegramBot, SHOW_COMMANDS};

mod callback_handler;
pub use callback_handler::*;

mod inline_keyboards;
pub use inline_keyboards::*;

mod commands;
pub use commands::*;

mod filters;
pub use filters::*;

pub async fn bot_init() {
    let bot: TelegramBot = Bot::new(crate::TOKEN_KEY)
    .parse_mode(teloxide::types::ParseMode::Html);

    if SHOW_COMMANDS {
        bot.set_my_commands(Command::bot_commands()).await.unwrap();
    }

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler))
        .branch(Update::filter_chat_member().endpoint(chat_member_handler));

    Dispatcher::builder(bot, handler).enable_ctrlc_handler().build().dispatch().await;
}