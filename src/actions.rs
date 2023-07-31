use std::str::FromStr;
use chrono::{DateTime, NaiveDateTime, Utc};
use rust_i18n::t;
use teloxide::{types::{MessageId, ChatId, UserId, ChatPermissions, True}, requests::Requester, payloads::RestrictChatMemberSetters};
use crate::TelegramBot;

pub enum Actions {
    DelMsg,
    DelAndKick,
    DelAndBan,
    ResolvedReadOnly,
    ResolvedReadOnlyForever,
    ResolvedNoMedia,
    ResolvedNoMediaForever,
    ResolvedNoStickers,
    ResolvedNoStickersForever,
    FalseAlarm,
    GiveWarn,
}

impl Actions {
    pub async fn invoke(&self, bot: TelegramBot, user_id: UserId, chat_id: ChatId, msg_id: MessageId , note: String) -> True {
        match self {
            Actions::DelMsg => bot.delete_message(chat_id, msg_id).await.unwrap(),
            Actions::DelAndKick => {
                bot.delete_message(chat_id, msg_id).await.unwrap();
                bot.kick_chat_member(chat_id, user_id).await.unwrap();
                True
            },
            Actions::DelAndBan => {
                bot.delete_message(chat_id, msg_id).await.unwrap();
                bot.ban_chat_member(chat_id, user_id).await.unwrap();
                True
            },
            Actions::ResolvedReadOnly => {
                let parsed_time = note.parse::<i64>().unwrap();
                let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_millis(Utc::now().timestamp_millis()+parsed_time*1000).unwrap(), Utc);
                bot.restrict_chat_member(chat_id, user_id, ChatPermissions::empty()).until_date(dt).await.unwrap();

                bot.delete_message(chat_id, msg_id).await.unwrap();
                True
            },
            Actions::ResolvedReadOnlyForever => {
                bot.restrict_chat_member(chat_id, user_id, ChatPermissions::empty()).await.unwrap();
                True
            },
            Actions::ResolvedNoMedia => {
                bot.restrict_chat_member(chat_id, user_id, ChatPermissions::SEND_MEDIA_MESSAGES).await.unwrap();
                True
            },
            Actions::ResolvedNoMediaForever => {
                bot.restrict_chat_member(chat_id, user_id, ChatPermissions::SEND_MEDIA_MESSAGES).await.unwrap();
                True
            },
            
            // **IMPORTANT - ResolvedNoStickers & ResolvedNoStickersForever are NOT WORKING**
            Actions::ResolvedNoStickers => {
                bot.restrict_chat_member(chat_id, user_id, ChatPermissions::SEND_OTHER_MESSAGES).await.unwrap();
                True
            },
            Actions::ResolvedNoStickersForever => todo!(),
            Actions::FalseAlarm => {
                True
            },
            Actions::GiveWarn => todo!(),
        }
    }
}

impl FromStr for Actions {
    type Err = ();

    fn from_str(s: &str) -> Result<Actions, ()> {
        match s {
            "DelMsg" => Ok(Actions::DelMsg),
            "DelAndKick" => Ok(Actions::DelAndKick),
            "DelAndBan" => Ok(Actions::DelAndKick),
            "DelAndReadOnly" => Ok(Actions::DelMsg),
            "ResolvedReadOnly" => Ok(Actions::ResolvedReadOnlyForever),
            "ResolvedReadOnlyForever" => Ok(Actions::ResolvedReadOnlyForever),
            "ResolvedNoMedia" => Ok(Actions::ResolvedNoMedia),
            "ResolvedNoMediaForever" => Ok(Actions::ResolvedNoMediaForever),
            "ResolvedNoStickers" => Ok(Actions::ResolvedNoStickersForever),
            "ResolvedNoStickersForever" => Ok(Actions::ResolvedNoStickersForever),
            "FalseAlarm" => Ok(Actions::FalseAlarm),
            "GiveWarn" => Ok(Actions::GiveWarn),
            _ => Err(()),
        }
    }
}

pub enum Punishments {
    DelMsg,
    DelAndBan,
    DelAndKick,
    DelAndReadOnly,
    NoMedia,
    NoStickers,
    NoViolations,
}

impl Punishments {
    pub fn get(&self) -> String{
        match self {
                Self::DelMsg => t!("punishments.buttons.delete"),
                Self::DelAndBan => t!("punishments.buttons.del_and_ban"),
                Self::DelAndReadOnly => t!("punishments.buttons.del_and_read_only"),
                Self::NoViolations => t!("punishments.buttons.no_violations"),
                Self::NoMedia => t!("punishments.buttons.no_media"),
                Self::NoStickers => t!("punishments.buttons.no_stickers"),
                Self::DelAndKick => t!("punishments.buttons.del_and_kick"),
        }
    }
}

impl FromStr for Punishments {
    type Err = ();

    fn from_str(s: &str) -> Result<Punishments, ()> {
        match s {
            "🗑 Delete" => Ok(Punishments::DelMsg),
            "🚫 Del&Ban" => Ok(Punishments::DelAndBan),
            "🙊 Del&Mut" => Ok(Punishments::DelAndReadOnly),
            "❎ No violations" => Ok(Punishments::NoViolations),
            "🖼 No media" => Ok(Punishments::NoMedia),
            _ => Err(()),
        }
    }
}

