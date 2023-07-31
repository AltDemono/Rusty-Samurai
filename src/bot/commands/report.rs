use crate::TelegramBot;

use rust_i18n::t;
use teloxide::prelude::*;

use crate::REPORTS_CHAT_ID;
use crate::bot::inline_keyboards::make_inline_punishment_keyboard;

// Report User
pub async fn report_user(bot: TelegramBot, msg: Message) -> ResponseResult<()> {
    let reply_message = Some(msg.reply_to_message()).unwrap_or(None);
    let is_group = msg.chat.is_chat();

    // is group?
    if is_group {
        match reply_message {
            Some(value) => {

                // Report Information
                let reported_user = reply_message.unwrap().from().unwrap();
                let user_sended_report = msg.from().unwrap();

                let is_bot = reported_user.is_bot;
                let user_id = reported_user.id;
                
                let admins = bot.get_chat_administrators(msg.chat.id).await.unwrap();
                let is_admin = admins.iter().any(|a| a.user.id == user_id);

                let id = ChatId {
                    0: REPORTS_CHAT_ID,
                };

                if !is_admin && is_group && !is_bot { 

                    // Report Info
                    let chat_title = msg.chat.title().unwrap();
                    let msg_text = reply_message.unwrap().text().unwrap();
                    let msg_url = value.url().unwrap();
                    let reported_user_name = reported_user.first_name.to_string();
                    let user_sended_report_name = user_sended_report.first_name.to_string();

                    let message = t!("messages.report_message", "chat" => chat_title, "msg"=>msg_text, "from"=>reported_user_name, "reported"=>user_sended_report_name,"url"=> msg_url);

                    // make keyboard
                    let keyboard = make_inline_punishment_keyboard(
                        reply_message.unwrap().id.to_string(), 
                        msg.chat.id, 
                        reported_user.id, 
                    );

                    let _ = bot.send_message(id, message).reply_markup(keyboard).await;
                    let _ = bot.send_message(msg.chat.id, t!("messages.report_sended")).reply_to_message_id(msg.id).await;
                }  else {
                    let _ = bot.send_message(msg.chat.id, t!("messages.cannot_report_user")).reply_to_message_id(msg.id).await;
                }

            },
            None => {let _ = bot.send_message(msg.chat.id, t!("messages.report_not_reply")).reply_to_message_id(msg.id).await;}
        }
    } else {
        let _ = bot.send_message(msg.chat.id, t!("messages.bot_isnt_in_chat")).reply_to_message_id(msg.id).await;
    }
    Ok(())
}
