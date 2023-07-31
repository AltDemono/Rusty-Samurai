use lingua::Language;
use rustrict::{Censor, Type};
use teloxide::requests::ResponseResult;
use teloxide::prelude::*;

use crate::{TelegramBot, bot_is_admin, get_language};

pub async fn chat_member_handler(bot: TelegramBot, m: ChatMemberUpdated) -> ResponseResult<()> {
    let (_, analysis) = Censor::from_str(m.new_chat_member.user.full_name().as_str())
        .with_censor_threshold(Type::INAPPROPRIATE)
        .with_censor_first_character_threshold(Type::OFFENSIVE & Type::SEVERE)
        .with_ignore_false_positives(false)
        .with_ignore_self_censoring(false)
        .with_censor_replacement('*')
        .censor_and_analyze();

    let chat_admins = bot.get_chat_administrators(m.chat.id);

    if analysis.is(Type::PROFANE & Type::SEVERE | Type::SEXUAL) && get_language(m.new_chat_member.user.full_name()) == Language::English &&
    bot_is_admin(bot.get_me().await.unwrap(), chat_admins, m.clone().chat).await  {
        bot.kick_chat_member(m.chat.id, m.new_chat_member.user.id).await.unwrap();
    }

    Ok(())
}