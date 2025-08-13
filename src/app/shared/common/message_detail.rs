pub mod message_detail {
    
    use actix_web::{ResponseError,HttpResponse,http::StatusCode};
    use serde::Serialize;
    use std::fmt;
    
    #[derive(Serialize,Debug)]
        pub struct ProblemDetails {
            pub r#type: Option<String>,
            pub title: String,
            pub status: u16,
            pub detail: String,
            pub instance: String
        }

    impl fmt::Display for ProblemDetails {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{} ({}): {} [{}]",
                self.title,
                self.status,
                self.detail,
                self.instance
            )?;

            if let Some(t) = &self.r#type {
                write!(f, " <{}>", t)?;
            }

            Ok(())
        }
    }


    impl ProblemDetails {
        pub fn new(r#type: Option<String>, title: String, status: u16, detail: String, instance: String) -> Self {
            ProblemDetails {
                r#type,
                title,
                status,
                detail,
                instance
            }
        }
        
        pub fn server_error(instance: String) -> Self {
            ProblemDetails::new(
                None,
                "Internal Server Error".to_string(),
                500,
                "An unexpected error occurred on the server.".to_string(),
                instance
            )
        }


        pub fn bad_request(detail: String,instance: String) -> Self {
            ProblemDetails::new(
                None,
                "Bad Request".to_string(),
                400,
                detail,
                instance
            )
        }
        
        pub fn not_found(instance: String) -> Self {
            ProblemDetails::new(
                None,
                "Not Found".to_string(),
                404,
                "The requested resource was not found.".to_string(),
                instance
            )
        }

        pub fn unprocesable_entity(instance: String, detail: String) -> Self {
            ProblemDetails::new(
                None,
                "Unprocesable Entity".to_string(),
                422,
                detail,
                instance
            )
        }
    }

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
}
