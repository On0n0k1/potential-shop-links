use actix_web::{
    get,
    web::ServiceConfig,
    web::{self, Data},
};
use dao::{user::User, Dao};
use log::info;
use mime::Mime;
use shuttle_actix_web::ShuttleActixWeb;
use std::sync::{Arc, Mutex};
use tera::Tera;

pub mod dao;
pub mod error;
pub mod globals;
pub mod server;

pub const TEMPLATEPATH: &str = "templates/**/*";

#[get("/static/{file:.*}")]
// Explicitly set the content type for JavaScript, WebAssembly, CSS, and image files
async fn serve_static(
    file: web::Path<String>,
    _: web::Data<Arc<Tera>>,
    _: web::Data<Arc<Mutex<Dao>>>,
) -> actix_web::Result<actix_files::NamedFile> {
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
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let tera: Tera = match Tera::new(TEMPLATEPATH) {
        Ok(tera) => tera,
        Err(err) => {
            panic!("Failed to parse templates: {err:?}");
        }
    };
    let tera: Arc<Tera> = Arc::new(tera);
    let dao: Dao = Dao::new(&secrets).await.unwrap();

    let dao: Arc<Mutex<Dao>> = Arc::new(Mutex::new(dao));
    info!("Resetting database admin...");
    User::reset(&dao, &secrets).await.unwrap();
    {
        info!("Inserting database admin...");
        // let username = "admin".to_string();
        // let password = "123456".to_string();
        let username = secrets.get("USERNAME").unwrap();
        let password = secrets.get("PASSWORD").unwrap();
        let user = User::create(&dao, username, password).await.unwrap();
        log::info!("User created: {user:?}");
    }

    // Create root user here
    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(Data::new(tera.clone()))
            .app_data(Data::new(dao.clone()))
            .service(server::web::index::index)
            .service(server::web::login::login)
            .service(server::user::auth::auth)
            .service(server::user::create::create)
            .service(serve_static)
            .service(server::proxy::proxy);
    };

    Ok(config.into())
}
