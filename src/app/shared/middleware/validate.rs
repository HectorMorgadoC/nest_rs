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
    use std::sync::Arc;

    pub async fn validate_role_admin(
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
                            if role == "admin" {
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

        /*
        // Usar trait para validar usuario
        match validator
            .validate_user_role_for_middleware(claims.user_identifier)
            .await
        {
            Ok(user_context) => {
                request.extensions_mut().insert(user_context);
                request.extensions_mut().insert(claims);
                Ok(request)
            }
            Err(_) => {
                let error =
                    ProblemDetails::unauthorized("User validation failed".to_string()).into();
                Err((error, request))
            }
        }
        */

        Ok(request)
    }
}
