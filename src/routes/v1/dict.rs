use crate::{
    error::{api_response, ApiResponse},
    models::{DictionaryHandle, IntoLanguageItem, Language, LanguageItem},
};
use chrono::{DateTime, Utc};
use rocket::{get, State};
use rocket_okapi::{openapi, JsonSchema};
use serde::Serialize;

#[openapi]
#[get("/languages")]
pub async fn get_languages() -> ApiResponse<Vec<Language>> {
    api_response(|| async { Ok(Language::entries()) }).await
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct GetStatusResponse {
    pub last_update: DateTime<Utc>,
    pub total_entries: u32,
    pub lang_progress: Vec<LanguageItem<u32>>,
}

#[openapi]
#[get("/status")]
pub async fn get_status(dict: &State<DictionaryHandle>) -> ApiResponse<GetStatusResponse> {
    api_response(|| async {
        let dict = dict.read().await;
        Ok(GetStatusResponse {
            last_update: dict.last_update.clone(),
            total_entries: dict.counters.total,
            lang_progress: dict.counters.lang_total.clone().into_vec(),
        })
    })
    .await
}
