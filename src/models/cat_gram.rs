use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, Eq, PartialEq)]
// #[serde(rename_all(SCREAMING_SNAKE_CASE))]
pub enum CatGram {
    VerbTransitive,
    VerbIntransitive,
    VerbReflexive,
    NounMasculineSingular,
    NounFeminineSingular,
    NounMasculinePlural,
    NounFemininePlural,
    NounMasculineInvariant,
    NounFeminineInvariant,
    Adjective,
    AdjectivePossessive,
    Adverb,
    ParticiplePast,
    ParticiplePresent,
    Preposition,
    PronounPersonal,
    PronounPossessive,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, Eq, PartialEq)]
pub enum Verb {
    Transitive,
    Intransitive,
    Reflexive,
}
