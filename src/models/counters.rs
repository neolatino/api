use rocket_okapi::JsonSchema;
use serde::Serialize;

/// The number of words in each language
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, JsonSchema)]
pub struct Counters {
    pub total: u32,
    pub sem: u32,
    pub lat: u32,
    pub iro: u32,
    pub por: u32,
    pub spa: u32,
    pub cat: u32,
    pub occ: u32,
    pub fra: u32,
    pub srd: u32,
    pub ita: u32,
    pub rom: u32,
    pub eng: u32,
    pub fol: u32,
    pub frk: u32,
    pub sla: u32,
}
