use crate::routes::*;
use models::Dictionary;
use rocket_okapi::{
    openapi_get_routes,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};

mod error;
mod models;
mod routes;

const DICT_URL: &str = "https://docs.google.com/spreadsheets/d/e/2PACX-1vSGCRUv8z5VSCk7lBy_4gtH2PkFvMH5ny65qauUmYzqWinGEw23IAQT_1seyBGfqw/pub?output=csv";

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let mut routes = openapi_get_routes![
        v1::dict::get_languages,
        v1::dict::get_status,
        v1::dict::get_entry,
        v1::dict::post_search,
    ];
    routes.append(&mut make_swagger_ui(&get_docs()).into());
    // routes.append(&mut routes!(debug));

    let dict = Dictionary::from_url(DICT_URL).await?;
    let rocket = rocket::build().manage(dict).mount("/", routes);

    rocket.launch().await?;

    Ok(())
}

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/openapi.json".to_string(),
        ..Default::default()
    }
}
