pub mod contract {

    use crate::app::shared::common::http_error::http_error::ProblemDetails;

    #[async_trait::async_trait]
    pub trait MiddlewareUserValidator {
        async fn validate_user_for_middleware(
            &self,
            _email: String,
        ) -> Result<MiddlewareUserContext, ProblemDetails>;
    }

    #[derive(Clone, Debug)]
    pub struct MiddlewareUserContext {
        pub email: String,
        pub roles: Vec<String>,
    }
}
