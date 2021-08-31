use std::path::PathBuf;

use maplit::hashmap;
use rosetta_build::RosettaConfig;
use unic_langid::langid;

#[test]
fn config_simple() -> Result<(), Box<dyn std::error::Error>> {
    let config = RosettaConfig::builder()
        .source("en", "translations/en.json")
        .source("fr", "translations/fr.json")
        .fallback("en")
        .build()?;

    let expected = RosettaConfig {
        fallback: (langid!("en"), PathBuf::from("translations/en.json")),
        others: hashmap! { langid!("fr") => PathBuf::from("translations/fr.json") },
        output: None,
    };

    assert_eq!(config, expected);

    Ok(())
}
