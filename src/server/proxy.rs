use actix_web::{post, web, HttpResponse, Responder};
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use log::{info, warn};
use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    url: String,
}

fn attempt_decode(text: &str) -> Option<Vec<u8>> {
    if let Ok(result) = general_purpose::STANDARD.decode(text) {
        info!("First succeeded");
        return Some(result);
    };
    if let Ok(result) =
        engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD).decode(text)
    {
        info!("Second succeeded");
        return Some(result);
    }
    None
}

#[post("/proxy")]
async fn proxy(info: web::Json<Info>) -> impl Responder {
    let info: Info = info.into_inner();
    log::info!("Proxy requested for url: {}", info.url);
    let response: Result<reqwest::Response, reqwest::Error> = reqwest::get(&info.url).await;

    match response {
        Ok(resp) => {
            log::info!("Successfully acquired page");
            let body: String = resp
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read response body".to_string());
            match attempt_decode(&body) {
                None => {
                    warn!("Failed to decrypt the body as Base64");
                    HttpResponse::Ok().body(body)
                }
                Some(decrypted) => {
                    info!("Successfully decrypted the body as base64");
                    match String::from_utf8(decrypted) {
                        Err(err) => {
                            warn!("Error parsing string: {err:?}");
                            HttpResponse::InternalServerError()
                                .body("Failed to parse the decrypted message as string")
                        }
                        Ok(decrypted) => HttpResponse::Ok().body(decrypted),
                    }
                }
            }
        }
        Err(err) => {
            log::warn!("Failed to retrieve page: {err:?}");
            HttpResponse::InternalServerError().body("Failed to fetch the URL")
        }
    }
}
