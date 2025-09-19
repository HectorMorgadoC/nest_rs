pub mod app_error {

    use diesel::result::Error as DieselError;
    use jsonwebtoken::errors::Error as JwtError;
    use serde_json::Error as SerdeJsonError;
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum AppError {
        // Errores de JWT
        #[error("Invalid or malformed JWT: {0}")]
        JwtError(#[from] JwtError),

        #[error("JWT secret key not configured")]
        JwtSecretNotConfigured,

        // Errores de base de datos (Diesel)
        #[error("Database error: {0}")]
        DatabaseError(#[from] DieselError),

        #[error("Missing environment variable: {0}")]
        MissingEnvVar(String),

        // Errores de serialización/deserialización JSON
        #[error("JSON error: {0}")]
        JsonError(#[from] SerdeJsonError),

        // Errores de negocio personalizados
        #[error("User not found")]
        UserNotFound,

        #[error("User is inactive")]
        UserInactive,

        #[error("Invalid issuer: expected {expected}, got {actual}")]
        InvalidIssuer { expected: String, actual: String },

        #[error("Internal server error: {0}")]
        InternalError(String),
    }

    /*
    // Implementar ResponseError para que Actix Web sepa cómo convertir AppError en respuesta HTTP
    impl ResponseError for AppError {
        fn status_code(&self) -> StatusCode {
            match self {
                AppError::JwtError(_) => StatusCode::UNAUTHORIZED,
                AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                AppError::JsonError(_) => StatusCode::BAD_REQUEST,
                AppError::UserNotFound | AppError::UserInactive => StatusCode::FORBIDDEN,
                AppError::InvalidIssuer { .. } => StatusCode::UNAUTHORIZED,
                AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            }
        }

        fn error_response(&self) -> HttpResponse {
            HttpResponse::build(self.status_code()).json(&ErrorResponse {
                error: self.to_string(),
            })
        }
    }
    */
}
