use teloxide::{prelude::*, utils::command::BotCommands, types::{Me, MessageKind}};
use rustrict::{Censor, Type};
use lingua::Language::{self};

pub use crate::utils::*;

mod report;
pub use report::*;

mod chat;
pub use chat::*;

use crate::TelegramBot;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Report the user to the owner of the bot")]
    Report,
    #[command(description = "Get invite link of this chat")]
    InviteLink,
    #[command(description = "Set a bot language")]
    SetLanguage(String)
}

pub async fn message_handler(
    bot: TelegramBot,
    msg: Message,
    me: Me,
) -> ResponseResult<()> {
    let kind = msg.clone().kind;

    match kind {
        MessageKind::NewChatMembers(_) => {
            bot.delete_message(msg.chat.id, msg.id).await.unwrap();
        }
        MessageKind::Common(_) => {
            match BotCommands::parse(msg.clone().text().unwrap(), me.username()) {
                Ok(Command::Report) => {
                    report_user(bot, msg).await.unwrap();
                },
                Ok(Command::InviteLink) => {
                    invite_link(bot, msg).await.unwrap();
                },
                Ok(Command::SetLanguage(lang)) => {
                    set_language(bot, msg, lang).await.unwrap();
                }
                Err(_) => {
                    common_hanler(msg, bot).await.unwrap();
                }
            }
        }
        MessageKind::LeftChatMember(_) => {bot.delete_message(msg.chat.id, msg.id).await.unwrap();},
        _ => {
            
        }
    }   
    Ok(())
}

async fn common_hanler(msg: Message, bot: TelegramBot) -> ResponseResult<()> {
    let (_, analysis) = Censor::from_str(msg.clone().text().unwrap())
    .with_censor_threshold(Type::INAPPROPRIATE)
    .with_censor_first_character_threshold(Type::OFFENSIVE & Type::SEVERE)
    .with_ignore_false_positives(false)
    .with_ignore_self_censoring(false)
    .with_censor_replacement('*')
    .censor_and_analyze();

    let me = bot.get_me().await.unwrap();
    let chat_admins = bot.get_chat_administrators(msg.chat.id);

    if analysis.is(Type::PROFANE & Type::SEVERE | Type::SEXUAL) && 
        get_language(msg.text().unwrap().to_string()) == Language::English && 
        bot_is_admin(me, chat_admins.clone(), msg.chat.clone()).await {
            bot.delete_message(msg.chat.id, msg.id).await.unwrap();
        } else if !bot_is_admin(bot.get_me().await.unwrap(),  bot.get_chat_administrators(msg.chat.id), msg.chat.clone()).await {
            bot.send_message(msg.chat.id, "❌ Bot is not an admin!").await.unwrap();
        }
        
    Ok(())
}