pub mod http_error {
    
    use actix_web::{ResponseError,HttpResponse,http::StatusCode,Result as ActixResult};
    use serde::Serialize;
    use std::fmt;
    
    pub type HttpError = ProblemDetails;
    pub type HttpResult<T> = ActixResult<T, HttpError>;


    #[derive(Serialize,Debug)]
        pub struct ProblemDetails {
            pub r#type: Option<String>,
            pub title: String,
            pub status: u16,
            pub detail: String,
        }

    impl fmt::Display for ProblemDetails {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{} ({}): {} ",
                self.title,
                self.status,
                self.detail,
            )?;

            if let Some(t) = &self.r#type {
                write!(f, " <{}>", t)?;
            }

            Ok(())
        }
    }

    impl std::error::Error for ProblemDetails {}
    
    impl ResponseError for ProblemDetails {
        fn status_code(&self) -> StatusCode {
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
        }
        
        fn error_response(&self) -> HttpResponse {
            HttpResponse::build(self.status_code())
                .content_type("application/problem+json")
                .json(self)
        }
    }


    impl ProblemDetails {
        pub fn new(r#type: Option<String>, title: String, status: u16, detail: String) -> Self {
            ProblemDetails {
                r#type,
                title,
                status,
                detail,
            }
        }
        
        pub fn server_error() -> Self {
            ProblemDetails::new(
                None,
                "Internal Server Error".to_string(),
                500,
                "An unexpected error occurred on the server.".to_string(),
            )

        }


        pub fn bad_request(detail: String) -> Self {
            ProblemDetails::new(
                None,
                "Bad Request".to_string(),
                400,
                detail,
            )

        }
        
        pub fn not_found() -> Self {
            ProblemDetails::new(
                None,
                "Not Found".to_string(),
                404,
                "The requested resource was not found.".to_string(),
            )

        }

        pub fn unprocesable_entity(detail: String) -> Self {
            ProblemDetails::new(
                None,
                "Unprocesable Entity".to_string(),
                422,
                detail,
            )

        }

        pub fn conflict(detail: String) -> Self {
            ProblemDetails::new ( 
                None, 
                "Conflict".to_string(), 
                409, 
                detail,  
            )
        }

        pub fn unauthorized(detail: String) -> Self {
             ProblemDetails::new ( 
                None, 
                "Unauthorized".to_string(), 
                401, 
                detail, 
            )
        }
        
        pub fn forbidden(detail: String) -> Self {
             ProblemDetails::new ( 
                None, 
                "Forbidden".to_string(), 
                403, 
                detail
            )
        }

        pub fn service_unavailable() -> Self {
             ProblemDetails::new ( 
                None, 
                "Service Unavailable".to_string(), 
                503, 
                "An unexpected error occurred on the server.".to_string(), 
            )

        }

        pub fn not_content(detail: String) -> Self {
                ProblemDetails::new ( 
                None, 
                "Not Content".to_string(), 
                204, 
                detail,  
            )

        }

        


    }

    /*
    impl ResponseError for ProblemDetails {
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn error_response(&self) -> HttpResponse {
        // Devuelve application/problem+json como recomienda RFC 7807
        HttpResponse::build(self.status_code())
            .content_type("application/problem+json")
            .json(self) // usa Serialize
    }
}
    */
}
