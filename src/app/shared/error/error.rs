pub mod error {
    use actix_web::{HttpResponse, ResponseError};
    use std::fmt;

    #[derive(Debug)]
    pub enum AppError {
        DatabaseError(String),
        ValidationError(String),
        NotFound(String),
        Unauthorized,
        InternalError(String),
    }

    impl fmt::Display for AppError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                AppError::DatabaseError(msg) => write!(f, "Database error: {msg}"),
                AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
                AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
                AppError::Unauthorized => write!(f, "Unauthorized"),
                AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            }
        }
    }

    impl ResponseError for AppError {
        fn error_response(&self) -> HttpResponse {
            match self {
                AppError::DatabaseError(_) => HttpResponse::InternalServerError().json("Internal server error"),
                AppError::ValidationError(msg) => HttpResponse::BadRequest().json(msg),
                AppError::NotFound(msg) => HttpResponse::NotFound().json(msg),
                AppError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
                _ => HttpResponse::InternalServerError().json("Internal server error"),
            }
        }
    }
}