#[cfg(test)]
mod tests {
    use lingua::{LanguageDetector, LanguageDetectorBuilder};
    use lingua::Language::{English, Russian, Ukrainian};

    #[test]
    fn language_detect() {
        let languages = vec![English, Russian, Ukrainian];
        let detector: LanguageDetector = LanguageDetectorBuilder::from_languages(&languages).build();

        assert_eq!(detector.detect_language_of("languages are awesome"), Some(English));
        assert_eq!(detector.detect_language_of("Русский язык"), Some(Russian));
        assert_eq!(detector.detect_language_of("Милозвучна українська мова"), Some(Ukrainian));
    }
}