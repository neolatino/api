use crate::models::LanguageCode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, JsonSchema, Serialize, Deserialize)]
pub struct Entry {
    pub id: u32,
    pub sem_id: Option<u32>,
    // pub cat_gram: CatGram,
    // pub theme: Theme,
    pub essential_flag: bool,
    pub basic_flag: bool,
    pub lat: Option<String>,
    pub iro: Option<String>,
    pub por: Option<String>,
    pub spa: Option<String>,
    pub cat: Option<String>,
    pub occ: Option<String>,
    pub fra: Option<String>,
    pub srd: Option<String>,
    pub ita: Option<String>,
    pub rom: Option<String>,
    pub eng: Option<String>,
    pub fol: Option<String>,
    pub frk: Option<String>,
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
