use actix_web::{http::StatusCode, HttpResponse, ResponseError};

#[derive(Debug)]
pub enum Error {
    TeraTemplatesParse(tera::Error),
    Database(mongodb::error::Error),
    UserNotFound,
    IncorrectPassword,
    ParsingOID(bson::oid::Error),
}

impl Error {
    pub fn tera_templates_parse<A>(err: tera::Error) -> Result<A, Self> {
        Err(Self::TeraTemplatesParse(err))
    }

    pub fn database<A>(err: mongodb::error::Error) -> Result<A, Self> {
        Err(Self::Database(err))
    }

    pub fn user_not_found<A>() -> Result<A, Self> {
        Err(Self::UserNotFound)
    }

    pub fn incorrect_password<A>() -> Result<A, Self> {
        Err(Self::IncorrectPassword)
    }

    pub fn parsing_oid<A>(err: bson::oid::Error) -> Result<A, Self> {
        Err(Self::ParsingOID(err))
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
            Self::IncorrectPassword => StatusCode::FORBIDDEN,
            Self::UserNotFound => StatusCode::NOT_FOUND,
            Self::ParsingOID(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let msg: String = match &self {
            Self::TeraTemplatesParse(err) => {
                format!("Failed to parse templates directory for tera: {err:?}")
            }
            Self::Database(err) => {
                format!("Database Error: {err:?}")
            }
            Self::UserNotFound => "Username does not exists".to_string(),
            Self::IncorrectPassword => "Password is incorrect".to_string(),
            Self::ParsingOID(err) => format!("Failed to parse Object Id: {err:?}"),
        };
        log::error!("{msg}");
        HttpResponse::build(self.status_code()).finish()
    }
}
