use crate::models::{LanguageCode, Topic};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, JsonSchema, Serialize, Deserialize)]
pub struct Entry {
    pub id: u32,
    pub sem_id: Option<u32>,
    // pub cat_gram: CatGram,
    pub topic: Option<Topic>,
    pub essential_flag: bool,
    pub basic_flag: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iro: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub por: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spa: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occ: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fra: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ita: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rom: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eng: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frk: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sla: Option<String>,
}

impl Entry {
    pub fn get(&self, lang: LanguageCode) -> Option<&str> {
        match lang {
            LanguageCode::NeoLatino => self.lat.as_ref(),
            LanguageCode::InterRomanico => self.iro.as_ref(),
            LanguageCode::Portuguese => self.por.as_ref(),
            LanguageCode::Spanish => self.spa.as_ref(),
            LanguageCode::Catalan => self.cat.as_ref(),
            LanguageCode::Occitan => self.occ.as_ref(),
            LanguageCode::French => self.fra.as_ref(),
            LanguageCode::Sardinian => self.srd.as_ref(),
            LanguageCode::Italian => self.ita.as_ref(),
            LanguageCode::Romanian => self.rom.as_ref(),
            LanguageCode::English => self.eng.as_ref(),
            LanguageCode::Folksprak => self.fol.as_ref(),
            LanguageCode::Frenkisch => self.frk.as_ref(),
            LanguageCode::InterSlavic => self.sla.as_ref(),
        }
        .map(|s| s.as_str())
    }

    pub fn matches(&self, text: &str, langs: &[LanguageCode]) -> bool {
        langs.iter().any(|l| self.get(*l) == Some(text))
    }
}
