//! Tests for rosetta i18n library
//!
//! This crate is only used to test code generated by rosetta-build
//! and does not expose anything useful.

#[cfg(test)]
mod tests {
    use std::{fmt::Debug, hash::Hash};

    use rosetta_i18n::{Language, LanguageId};
    use static_assertions::assert_impl_all;

    rosetta_i18n::include_translations!();

    assert_impl_all!(
        Lang: Language,
        Debug,
        Clone,
        Copy,
        Eq,
        PartialEq,
        Hash,
        Send,
        Sync
    );

    #[test]
    fn test_simple() {
        assert_eq!(Lang::En.hello(), "Hello world!");
        assert_eq!(Lang::Fr.hello(), "Bonjour le monde !");
    }

    #[test]
    fn test_formatted() {
        assert_eq!(Lang::En.hello_name("John"), "Hello John!");
        assert_eq!(Lang::Fr.hello_name("John"), "Bonjour John !");
    }

    #[test]
    fn test_formatted_multiple() {
        assert_eq!(Lang::En.display_age(30, "John"), "John is 30 years old.");
        assert_eq!(Lang::Fr.display_age(30, "John"), "John a 30 ans.");
    }

    #[test]
    fn test_fallback() {
        assert_eq!(Lang::Fr.fallback_key(), Lang::En.fallback_key());
        assert_eq!(Lang::fallback(), Lang::En);
    }

    #[test]
    fn test_from_language_id() {
        let en = LanguageId::new("en");
        let fr = LanguageId::new("fr");
        let de = LanguageId::new("de");

        assert_eq!(Lang::from_language_id(&en), Some(Lang::En));
        assert_eq!(Lang::from_language_id(&fr), Some(Lang::Fr));
        assert_eq!(Lang::from_language_id(&de), None);
    }

    #[test]
    fn test_to_language_id() {
        assert_eq!(Lang::En.language_id().value(), "en");
        assert_eq!(Lang::Fr.language_id().value(), "fr");
    }
}
