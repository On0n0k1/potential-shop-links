use crate::dao::entry::Entry;
use crate::dao::Dao;
use crate::error::Error;
use actix_web::{post, web, HttpResponse};
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tera::Tera;

#[derive(Deserialize, Serialize)]
pub struct Request {
    data: String,
}

#[post("/entry/{id}")]
pub async fn create(
    dao: web::Data<Arc<Mutex<Dao>>>,
    _: web::Data<Arc<Tera>>,
    id: web::Path<String>,
    req: web::Json<Request>,
) -> Result<HttpResponse, Error> {
    let req = req.into_inner();
    let id = id.into_inner();
    let data = req.data;
    let user_id = ObjectId::parse_str(id).or_else(Error::parsing_oid)?;
    // Validate username later
    // let _username = username.into_inner();
    Entry::create(&dao, user_id, data).await?;
    Ok(HttpResponse::Ok().finish())
}
