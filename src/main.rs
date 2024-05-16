use std::sync::Arc;

use actix_web::{
    get,
    web::ServiceConfig,
    web::{self, Data},
};
use dao::Dao;
use mime::Mime;
use shuttle_actix_web::ShuttleActixWeb;
use tera::Tera;

pub mod dao;
pub mod error;
pub mod server;

pub const TEMPLATEPATH: &str = "templates/**/*";

#[get("/static/{file:.*}")]
// Explicitly set the content type for JavaScript, WebAssembly, CSS, and image files
async fn serve_static(file: web::Path<String>) -> actix_web::Result<actix_files::NamedFile> {
    let file_path = format!("./static/{}", file);
    let content_type: Mime = if file_path.ends_with(".wasm") {
        "application/wasm".parse().unwrap()
    } else if file_path.ends_with(".css") {
        "text/css".parse().unwrap()
    } else if file_path.ends_with(".jpg") || file_path.ends_with(".jpeg") {
        "image/jpeg".parse().unwrap()
    } else if file_path.ends_with(".png") {
        "image/png".parse().unwrap()
    } else if file_path.ends_with(".gif") {
        "image/gif".parse().unwrap()
    } else if file_path.ends_with(".json") {
        "application/json".parse().unwrap()
    } else {
        "application/javascript".parse().unwrap() // Default to JavaScript for unrecognized file types
    };
    log::info!("Retrieving {content_type} file");
    Ok(actix_files::NamedFile::open(file_path)?.set_content_type(content_type))
}

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
            .service(server::web::index::index)
            .service(serve_static);
    };

    Ok(config.into())
}
