use enum_map::Enum;
use rocket::FromFormField;
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
    FromFormField,
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
