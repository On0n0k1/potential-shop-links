use crate::dao::Dao;
use actix_web::{get, web, HttpResponse, Responder};
use std::sync::Arc;
use tera::Context;

#[get("/")]
async fn index(
    tera: web::Data<Arc<tera::Tera>>,
    _: web::Data<Arc<Dao>>,
) -> Result<impl Responder, actix_web::Error> {
    log::info!("Web Index called");
    let context: Context = tera::Context::new();
    let rendered: String = tera.render("index.html", &context).map_err(|e| {
        log::error!("{e:?}");
        actix_web::error::ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered))
}
