pub mod message_detail {
    
    use serde::Serialize;
    
    #[derive(Serialize)]
        pub struct ProblemDetails {
            pub r#type: String,
            pub title: String,
            pub status: u16,
            pub detail: String,
            pub instance: String
        }
    impl ProblemDetails {
        pub fn new(r#type: String, title: String, status: u16, detail: String, instance: String) -> Self {
            ProblemDetails {
                r#type,
                title,
                status,
                detail,
                instance
            }
        }
        
        pub fn server_error(_type: String) -> Self {
            ProblemDetails::new(
                _type,
                "Internal Server Error".to_string(),
                500,
                "An unexpected error occurred on the server.".to_string(),
                "/".to_string()
            )
        }
        
        pub fn not_found(_type: String) -> Self {
            ProblemDetails::new(
                _type,
                "Not Found".to_string(),
                404,
                "The requested resource was not found.".to_string(),
                "/".to_string()
            )
        }
    }
}