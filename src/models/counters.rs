use crate::models::LanguageCode;
use enum_map::EnumMap;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Counters {
    pub total: u32,
    pub sem_total: u32,
    pub lang_total: EnumMap<LanguageCode, u32>,
}

impl Counters {
    pub fn lang_progress(&self) -> EnumMap<LanguageCode, f32> {
        self.lang_total.map(|_, v| v as f32 / self.total as f32)
    }
}
