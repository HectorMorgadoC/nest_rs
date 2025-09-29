pub mod validate_role {
    // app/shared/middleware/authentication/mod.rs
    use crate::app::{
        config::env::env::Env,
        shared::{
            common::http_error::http_error::ProblemDetails,
            contract::contract::{MiddlewareUserContext, MiddlewareUserValidator},
            middleware::authentication::json_web_token::Claims,
        },
    };

    use actix_web::{Error, dev::ServiceRequest, web};
    use actix_web_httpauth::extractors::bearer::BearerAuth;
    use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
    use std::{pin::Pin, sync::Arc};

    pub fn role_validator(
        required_roles: Vec<String>,
    ) -> impl Fn(
        ServiceRequest,
        BearerAuth,
    ) -> Pin<
        Box<dyn Future<Output = Result<ServiceRequest, (actix_web::Error, ServiceRequest)>>>,
    > {
        move |request: ServiceRequest, credentials: BearerAuth| {
            let required_roles = required_roles.clone();
            Box::pin(async move {
                validate_role_internal(required_roles.clone(), request, credentials).await
            })
        }
    }

    pub async fn validate_role_internal(
        required_roles: Vec<String>,
        request: ServiceRequest,
        credentials: BearerAuth,
    ) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
        // Obtener el validador desde app_data
        let validator = match request.app_data::<web::Data<Arc<dyn MiddlewareUserValidator>>>() {
            Some(data) => data,
            None => {
                let error: Error = ProblemDetails::server_error().into();
                return Err((error, request)); // request disponible aquí
            }
        };

        let validation = Validation::new(Algorithm::HS512);
        let enviroment_variables = Env::init();
        let key = enviroment_variables.get("KEY");

        if let Some(_key) = key {
            let decoding_key = DecodingKey::from_secret(_key.as_ref());
            let token_data = decode::<Claims>(&credentials.token(), &decoding_key, &validation);

            if let Ok(data) = token_data {
                let email = data.claims.subject;

                match validator.validate_user_role_for_middleware(email).await {
                    Ok(context) => {
                        let context_roles = context.roles;

                        for role in context_roles {
                            if required_roles.contains(&role) {
                                return Ok(request);
                            }
                        }

                        return Err((
                            ProblemDetails::unauthorized("Invalid role".to_string()).into(),
                            request,
                        ));
                    }
                    Err(err) => return Err((err.into(), request)),
                }
            }
        }

        Ok(request)
    }
}
