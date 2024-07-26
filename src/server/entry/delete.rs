use crate::dao::entry::Entry;
use crate::dao::Dao;
use crate::error::Error;
use actix_web::{post, web, HttpResponse};
use bson::oid::ObjectId;
use std::sync::{Arc, Mutex};
use tera::Tera;

#[post("/entry/{id}")]
pub async fn create(
    dao: web::Data<Arc<Mutex<Dao>>>,
    _: web::Data<Arc<Tera>>,
    id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let id = id.into_inner();
    let id = ObjectId::parse_str(id).or_else(Error::parsing_oid)?;
    Entry::delete(&dao, id).await?;
    Ok(HttpResponse::Ok().finish())
}
