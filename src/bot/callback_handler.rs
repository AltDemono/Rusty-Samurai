use rust_i18n::t;
use teloxide::{types::{MessageId, Message, CallbackQuery, ChatId, UserId}, requests::{Requester, ResponseResult}, payloads::EditMessageReplyMarkupSetters};

use crate::{actions::{Punishments, Actions}, REPORTS_CHAT_ID, TelegramBot, make_inline_time_picker_keyboard};

/// When it receives a callback from a button it edits the message with all
/// those buttons writing a text with the selected Debian version.
///
/// **IMPORTANT**: do not send privacy-sensitive data this way!!!
/// Anyone can read data stored in the callback button.
pub async fn callback_handler(bot: TelegramBot, q: CallbackQuery) -> ResponseResult<()> {
    if let Some(punishment_d) = q.data {
        let text = format!("Punishment: {}\nConfirm the action.", punishment_d); 

        bot.answer_callback_query(q.id).await?;

        // Edit text of the message to which the buttons were attached
        if let Some(Message { id, chat,.. }) = q.message {

            let split_input = punishment_d.split("/");
            let output: Vec<&str> = split_input.collect();
            
            match output[0].parse::<i64>().unwrap() {
                1 => {
                    // Chat id from which the message was sent
                    let chat_id = ChatId {
                        0: output[1].parse::<i64>().unwrap(),
                    };

                    // Reported message id
                    let reported_msg_id = MessageId { 
                        0: output[2].parse::<i32>().unwrap(),
                    };

                    let reported_user: UserId = UserId {
                        0: output[3].parse::<u64>().unwrap(),
                    };

                    // Reports chat id
                    let current_chat_id = ChatId {
                        0: REPORTS_CHAT_ID,
                    };

                    let punishment = output[4];
                    match punishment.parse::<Punishments>().unwrap() {
                        Punishments::DelMsg => {
                            bot.clone().delete_message(chat.id, id).await?;
                            Actions::DelMsg.invoke(bot, reported_user, chat_id, reported_msg_id, "".to_string()).await;
                        },
                        Punishments::DelAndBan => {
                            Actions::DelAndBan.invoke(bot.clone(), reported_user, current_chat_id, reported_msg_id, "".to_string()).await;
                            bot.clone().delete_message(chat.id, id).await?;
                            bot.clone().send_message(chat.id, t!("messages.user_was_baned")).await?;
                        },
                        Punishments::DelAndReadOnly => {

                            let kb = make_inline_time_picker_keyboard(&punishment_d);
                            bot.clone().edit_message_reply_markup(chat.id, id).reply_markup(kb).await?;

                            if output.len() > 5 && output[5] != "" {
                                Actions::ResolvedReadOnly.invoke(bot.clone(), reported_user, chat_id, reported_msg_id, output[5].to_string()).await;
                                bot.clone().delete_message(chat.id, id).await?;
                                bot.clone().send_message(chat.id, t!("messages.user_was_muted")).await?;
                            }
                        },
                        Punishments::NoMedia => {
                            bot.clone().delete_message(chat.id, id).await?;
                            Actions::ResolvedNoMedia.invoke(bot, reported_user, chat_id, reported_msg_id, "".to_string()).await;
                        },
                        Punishments::NoStickers => {
                            bot.clone().delete_message(chat.id, id).await?;
                            Actions::ResolvedNoStickers.invoke(bot, reported_user, chat_id, reported_msg_id, "".to_string()).await;
                        },
                        Punishments::NoViolations => {
                            let kb = make_inline_time_picker_keyboard(&punishment_d);

                            bot.clone().edit_message_reply_markup(chat.id, id).reply_markup(kb).await?;
                            bot.clone().delete_message(chat.id, id).await?;
                            if output.len() > 5 && output[5] != "" {
                                Actions::FalseAlarm.invoke(bot.clone(), reported_user, chat_id, reported_msg_id, output[5].to_string()).await;
                                bot.clone().delete_message(chat.id, id).await?;
                                bot.clone().send_message(chat.id, t!("messages.user_was_muted")).await?;
                            }
                        },
                        Punishments::DelAndKick => { 
                            Actions::DelAndKick.invoke(bot.clone(), reported_user, current_chat_id, reported_msg_id, "".to_string()).await;
                            bot.clone().delete_message(chat.id, id).await?;
                            bot.clone().send_message(chat.id, t!("messages.user_was_kicked")).await?;
                        },
                    };
                },
                i64::MIN..=0_i64 => todo!(),
                2_i64..=i64::MAX => todo!(),
            }
        } else if let Some(id) = q.inline_message_id {
            bot.edit_message_text_inline(id, text).await?;
        }
    }

    Ok(())
}