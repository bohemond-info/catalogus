use rocket::serde::json::Json;
use rocket_okapi::openapi;
use data_transcriber::{convert, Catalogus};

#[openapi]
#[get("/catalogus")]
pub fn get_catalogus() -> Option<Json<Catalogus>> {
    match convert("resources/catalogus.csv".to_string()) {
        Ok(catalogus) => Some(Json(catalogus)),
        _ => None
    }
}
