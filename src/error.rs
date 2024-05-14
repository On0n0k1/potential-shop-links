use actix_web::{http::StatusCode, HttpResponse, ResponseError};

#[derive(Debug)]
pub enum Error {
    DatabaseInitializationError(std::io::Error),
    DatabaseConnectionFailed(sqlx::error::Error),
    TeraTemplatesParse(tera::Error),
}

impl Error {
    pub fn database_initialization_error<A>(err: std::io::Error) -> Result<A, Self> {
        Err(Self::DatabaseInitializationError(err))
    }

    pub fn database_connection_failed<A>(err: sqlx::error::Error) -> Result<A, Self> {
        Err(Self::DatabaseConnectionFailed(err))
    }

    pub fn tera_templates_parse<A>(err: tera::Error) -> Result<A, Self> {
        Err(Self::TeraTemplatesParse(err))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        #[allow(clippy::match_single_binding)]
        match &self {
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let msg: String = match &self {
            Self::DatabaseInitializationError(err) => {
                format!("Failed to initialize SQlite Database: {err:?}")
            }
            Self::DatabaseConnectionFailed(err) => {
                format!("Failed to connect to database: {err:?}")
            }
            Self::TeraTemplatesParse(err) => {
                format!("Failed to parse templates directory for tera: {err:?}")
            }
        };
        log::error!("{msg}");
        HttpResponse::build(self.status_code()).finish()
    }
}
