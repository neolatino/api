use enum_map::Enum;
use rocket::FromFormField;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

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
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Topic {
    #[serde(alias = "agricultura")]
    Agriculture,
    #[serde(alias = "alimentatione")]
    Food,
    #[serde(alias = "arte et cultura")]
    ArtAndCulture,
    #[serde(alias = "casa")]
    Home,
    #[serde(alias = "circumstantias")]
    Circumstance,
    #[serde(alias = "civtates et villas")]
    City,
    #[serde(alias = "communicatione")]
    Communication,
    #[serde(alias = "còrpo humano")]
    Body,
    #[serde(alias = "èssere humano")]
    Human,
    #[serde(alias = "grammàtica")]
    Grammar,
    #[serde(alias = "indústria")]
    Industry,
    #[serde(alias = "materiales et substantsas")]
    Materials,
    #[serde(alias = "natura")]
    Nature,
    #[serde(alias = "objèctos,apparatos, màchinas")]
    ObjectAndMachine,
    #[serde(alias = "perceptione")]
    Perception,
    #[serde(alias = "scièntias")]
    Science,
    #[serde(alias = "societate")]
    Society,
    #[serde(alias = "tecnología")]
    Technology,
    #[serde(alias = "tèmpo")]
    Time,
    #[serde(alias = "viaggîare")]
    Transport,
}
