use crate::dao::Dao;
use actix_web::{get, web, HttpResponse, Responder};
use std::sync::{Arc, Mutex};
use tera::Context;

#[get("/profile/{username}")]
async fn index(
    tera: web::Data<Arc<tera::Tera>>,
    _: web::Data<Arc<Mutex<Dao>>>,
    username: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    log::info!("Web Index called");
    let _ = username.into_inner();
    let context: Context = tera::Context::new();
    let rendered: String = tera.render("index.html", &context).map_err(|e| {
        log::error!("{e:?}");
        actix_web::error::ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered))
}
