// Lingua
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use lingua::Language::{English, Russian, Ukrainian};

// Teloxide
use teloxide::payloads::GetChatAdministrators;
use teloxide::requests::JsonRequest;
use teloxide::types::{Chat, Me};

pub fn get_language(text: String) -> Language {
    let languages = vec![English, Russian, Ukrainian];
    let detector: LanguageDetector = LanguageDetectorBuilder::from_languages(&languages).build();

    return detector.detect_language_of(&text).unwrap()
} 

pub async fn bot_is_admin(me: Me, admins: JsonRequest<GetChatAdministrators>, chat: Chat) -> bool {
    if chat.is_chat() {
        let bot_id = me.id;
        let is_admin = admins.await.unwrap().iter().any(|a| a.user.id == bot_id);

        return is_admin;
    } 
    false
}
