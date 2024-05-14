use std::sync::Arc;

use actix_web::{web::Data, web::ServiceConfig};
use dao::Dao;
use shuttle_actix_web::ShuttleActixWeb;
use tera::Tera;

pub mod dao;
pub mod error;
pub mod server;

pub const TEMPLATEPATH: &str = "templates/**/*";

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let tera: Tera = match Tera::new(TEMPLATEPATH) {
        Ok(tera) => tera,
        Err(err) => {
            panic!("Failed to parse templates: {err:?}");
        }
    };
    let tera: Arc<Tera> = Arc::new(tera);
    let dao: Dao = Dao::new().await.unwrap();
    let dao: Arc<Dao> = Arc::new(dao);
    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(Data::new(tera.clone()))
            .app_data(Data::new(dao.clone()))
            .service(server::web::index::index);
    };

    Ok(config.into())
}
