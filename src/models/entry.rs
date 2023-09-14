use crate::models::LanguageCode;
use enum_map::EnumMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub id: u32,
    // pub cat_gram: CatGram,
    // pub theme: Theme,
    pub data: EnumMap<LanguageCode, Option<String>>,
}
