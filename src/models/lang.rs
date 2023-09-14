use enum_map::{Enum, EnumMap};
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(
    Serialize,
    Deserialize,
    JsonSchema,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Enum,
    EnumIter,
    Display,
    Hash,
)]
pub enum LanguageCode {
    #[serde(rename = "LAT")]
    NeoLatino,
    #[serde(rename = "IRO")]
    InterRomanico,
    #[serde(rename = "POR")]
    Portuguese,
    #[serde(rename = "SPA")]
    Spanish,
    #[serde(rename = "CAT")]
    Catalan,
    #[serde(rename = "OCC")]
    Occitan,
    #[serde(rename = "FRA")]
    French,
    #[serde(rename = "SRD")]
    Sardinian,
    #[serde(rename = "ITA")]
    Italian,
    #[serde(rename = "ROM")]
    Romanian,
    #[serde(rename = "ENG")]
    English,
    #[serde(rename = "FOL")]
    Folksprak,
    #[serde(rename = "FRK")]
    Frenkisch,
    #[serde(rename = "SLA")]
    InterSlavic,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, Eq, PartialEq)]
pub struct Language {
    pub code: LanguageCode,
    pub name: String,
}

impl Language {
    pub fn new(code: LanguageCode, name: String) -> Self {
        Self { code, name }
    }

    pub fn entries() -> Vec<Language> {
        LanguageCode::iter()
            .map(|code| Language::new(code, code.to_string()))
            .collect()
    }
}

#[derive(Serialize, JsonSchema, Clone, Debug)]
pub struct LanguageItem<T: Serialize + JsonSchema + Clone + Debug> {
    pub code: LanguageCode,
    pub value: T,
}

pub trait IntoLanguageItem {
    type Item;
    fn into_vec(self) -> Vec<LanguageItem<Self::Item>>
    where
        <Self as IntoLanguageItem>::Item: Serialize + JsonSchema + Clone + Debug;
}

impl<T: Serialize + JsonSchema + Clone + Debug> IntoLanguageItem for EnumMap<LanguageCode, T> {
    type Item = T;

    fn into_vec(self) -> Vec<LanguageItem<Self::Item>> {
        self.into_iter()
            .map(|(code, value)| LanguageItem { code, value })
            .collect()
    }
}
