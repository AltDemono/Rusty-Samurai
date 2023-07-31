use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, UserId, ChatId};

use crate::actions::Punishments;

// Make punishment keyboard
pub fn make_inline_punishment_keyboard(reported_msg_id: String, chat: ChatId, reported_user: UserId) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let buttons = [
        Punishments::DelMsg.get(),
        Punishments::DelAndBan.get(),
        Punishments::DelAndReadOnly.get(),
        Punishments::NoViolations.get(),
        Punishments::NoMedia.get(),
        Punishments::NoStickers.get(),
        Punishments::DelAndKick.get(),
    ];

    for buttns in buttons.chunks(3) {
        let row = buttns
            .iter()
            .map(|button| InlineKeyboardButton::callback(button.to_owned(), format!("1/{chat}/{reported_msg_id}/{reported_user}/{button}")))
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

// Make a time picker keyboard
pub fn make_inline_time_picker_keyboard(data: &String) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
 
    let buttons: Vec<String> = crate::BLOCK_TIMES.into_iter()
                    .map(|i64| i64.to_string())
                    .collect();

    for buttns in buttons.chunks(3) {
        let row = buttns
            .iter()
            .map(|button| InlineKeyboardButton::callback(button.to_owned(), format!("{data}/{button}")))
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}