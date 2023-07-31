use rust_i18n::t;
use teloxide::{requests::{ResponseResult, Requester}, types::{ChatKind, Message}, payloads::SendMessageSetters};

use crate::{TelegramBot, OWNER_ID};

pub async fn invite_link(bot: TelegramBot, msg: Message) -> ResponseResult<()> {
   match msg.chat.kind {
        ChatKind::Private(_) => {
            bot.send_message(msg.chat.id, t!("messages.failed_send_message_private_chat")).reply_to_message_id(msg.id).await?;
        },
        ChatKind::Public(_) => {
            let chat = bot.get_chat(msg.chat.id).await?;
            let invite_link = chat.invite_link().unwrap();
            let text = t!("messages.invite_link_message", "invite_link" => invite_link);

            bot.send_message(msg.chat.id, text).reply_to_message_id(msg.id).await?;
        }
    }

    Ok(())
}

pub async fn set_language(bot: TelegramBot, msg: Message, lang: String) -> ResponseResult<()> {
    if msg.from().unwrap().id.0 == OWNER_ID {
        match lang.as_str() {
            "en-us" => {
                rust_i18n::set_locale("en-us");
                bot.send_message(msg.chat.id, t!("messages.language_changed")).await.unwrap();    
            }
            "ru-ru" => {
                rust_i18n::set_locale("ru-ru");
                bot.send_message(msg.chat.id, t!("messages.language_changed")).await.unwrap();    
            }
            "uk-ua" => {
                rust_i18n::set_locale("uk-ua");
                bot.send_message(msg.chat.id, t!("messages.language_changed")).await.unwrap();    
            }
            _ => {
                bot.send_message(msg.chat.id, t!("messages.failed_set_language")).await.unwrap();    
            }
        }
    } else {
        bot.send_message(msg.chat.id, t!("messages.not_owner")).await.unwrap();
    }

    Ok(())
}