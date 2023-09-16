use crate::{
    error::{api_response, ApiResponse},
    models::{Counters, DictionaryHandle, Entry, Language, LanguageCode, Topic},
};
use chrono::{DateTime, Utc};
use rocket::{get, post, serde::json::Json, State};
use rocket_okapi::{openapi, JsonSchema};
use serde::{Deserialize, Serialize};

#[openapi]
#[get("/languages")]
pub async fn get_languages() -> ApiResponse<Vec<Language>> {
    api_response(|| async { Ok(Language::entries()) }).await
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct GetStatusResponse {
    pub last_update: DateTime<Utc>,
    pub counters: Counters,
}

#[openapi]
#[get("/status")]
pub async fn get_status(dict: &State<DictionaryHandle>) -> ApiResponse<GetStatusResponse> {
    api_response(|| async {
        let dict = dict.read().await;
        Ok(GetStatusResponse {
            last_update: dict.last_update,
            counters: dict.counters.clone(),
        })
    })
    .await
}
#[openapi]
#[get("/entry/<id>")]
pub async fn get_entry(dict: &State<DictionaryHandle>, id: u32) -> ApiResponse<Entry> {
    api_response(|| async {
        let dict = dict.read().await;
        dict.get_entry(id)
    })
    .await
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct SearchFilters {
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    text_lang: Vec<LanguageCode>,
    #[serde(default)]
    sem_id: Option<u32>,
    #[serde(default)]
    topics: Vec<Topic>,
}

#[openapi]
#[post("/search", data = "<filters>")]
pub async fn post_search(
    dict: &State<DictionaryHandle>,
    filters: Json<SearchFilters>,
) -> ApiResponse<Vec<Entry>> {
    api_response(|| async {
        let dict = dict.read().await;
        dict.search(
            filters.text.clone(),
            filters.text_lang.clone(),
            filters.sem_id,
            filters.topics.clone(),
        )
    })
    .await
}
