use crate::dao::{user::User, Dao};
use crate::error::Error;
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tera::Tera;

#[derive(Deserialize, Serialize)]
pub struct Request {
    password: String,
}

#[post("/user/{username}")]
pub async fn auth(
    dao: web::Data<Arc<Mutex<Dao>>>,
    _: web::Data<Arc<Tera>>,
    username: web::Path<String>,
    req: web::Json<Request>,
) -> Result<HttpResponse, Error> {
    let req = req.into_inner();
    let password = req.password;
    let username = username.into_inner();
    User::authenticate(&dao, username, password).await?;
    Ok(HttpResponse::Ok().finish())
}
