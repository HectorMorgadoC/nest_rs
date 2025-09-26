pub mod json_web_token {
    use crate::app::{
        config::env::env::Env,
        shared::common::{
            error::app_error::AppError,
            http_error::http_error::{self, ProblemDetails},
        },
    };
    use actix_web::{
        body::{BoxBody, MessageBody},
        dev::{ServiceFactory, ServiceRequest, ServiceResponse},
        http::header::AUTHORIZATION,
        middleware::Next,
    };

    use actix_web_httpauth::extractors::bearer::BearerAuth;

    use chrono::{Duration, Utc};
    use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
    use serde::{Deserialize, Serialize};

    // La norma de jwt obliga a que los campos de la estructura claim vallan asi (iss,sub,exp,iat)
    // por eso de usa el #[serde(rename:--)] de lo contrario generan un mal token y al verificarlo
    // dara error
    //
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Claims {
        #[serde(rename = "iss")]
        issuer: String,
        #[serde(rename = "sub")]
        subject: String,
        #[serde(rename = "exp")]
        expiration: usize,
        #[serde(rename = "iat")]
        issued_at: usize,
        user_id: usize,
    }

    #[derive(Debug)]
    pub struct JwtClaim {
        pub issuer: String,
        pub subject: String,
        pub user_id: usize,
    }

    impl JwtClaim {
        pub fn new(issuer: String, subject: String, user_id: usize) -> Self {
            Self {
                issuer,
                subject,
                user_id,
            }
        }

        pub fn generate_token(&self) -> Result<String, AppError> {
            let enviroment_variables = Env::init();
            let key = enviroment_variables.get("KEY");
            let token_time: i16 = enviroment_variables
                .get_parsed("TOKEN_TIME")
                .unwrap_or_else(|| 200);
            let header = Header::new(Algorithm::HS512);
            if let Some(_key) = key {
                let encoding_key = EncodingKey::from_secret(_key.as_ref());
                let _issued_at =
                    (Utc::now() + Duration::minutes(token_time as i64)).timestamp() as usize;
                let _expiration = Utc::now().timestamp() as usize;
                println!("Creacion: {}, Expiracion: {}", _issued_at, _expiration);

                let claims: Claims = Claims {
                    issuer: self.issuer.clone(),
                    subject: self.subject.clone(),
                    expiration: _expiration,
                    issued_at: _issued_at,
                    user_id: self.user_id,
                };
                encode(&header, &claims, &encoding_key).map_err(AppError::JwtError)
            } else {
                Err(AppError::JwtSecretNotConfigured)
            }
        }
    }

    pub fn extract_token_from_request(request: &ServiceRequest) -> Result<String, AppError> {
        let auth_header =
            request
                .headers()
                .get(AUTHORIZATION)
                .ok_or_else(|| AppError::Unauthorized {
                    message_error: "Authorization token required".to_string(),
                })?;

        let auth_str = auth_header.to_str().map_err(|_| AppError::Unauthorized {
            message_error: "Invalid authorization header".to_string(),
        })?;

        if !auth_str.starts_with("Bearer ") {
            return Err(AppError::Unauthorized {
                message_error: "Invalid token format. Use: Bearer <token>".to_string(),
            });
        }

        let token = auth_str.trim_start_matches("Bearer ").to_string();

        if token.is_empty() {
            return Err(AppError::Unauthorized {
                message_error: "Empty token".to_string(),
            });
        }

        Ok(token)
    }

    /*
    pub async fn auth_middleware(
        request: ServiceRequest,
        next: Next<impl ServiceFactory<ServiceRequest>>,
    ) -> Result<ServiceResponse<ServiceRequest>, Error> {
        let token = extract_token_from_request(&request);
        let validation = Validation::new(Algorithm::HS512);
        let enviroment_variables = Env::init();
        let key = enviroment_variables.get("KEY");

        if let Some(_key) = key {
            let decoding_key = DecodingKey::from_secret(_key.as_ref());
            match token {
                Ok(value) => {
                    let validation_token = decode::<Claims>(&value, &decoding_key, &validation);
                    if let Err(_) = validation_token {
                        return Err(
                            ProblemDetails::unauthorized("Invalid token".to_string()).into()
                        );
                    }
                }
                Err(_) => {
                    return Err(ProblemDetails::unauthorized("Invalid token".to_string()).into());
                }
            }
        } else {
            return Err(ProblemDetails::server_error().into());
        }

        next.call(request).await
    }
    */

    pub async fn validator(
        request: ServiceRequest,
        credential: BearerAuth,
    ) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
        let validation = Validation::new(Algorithm::HS512);
        let enviroment_variables = Env::init();
        let key = enviroment_variables.get("KEY");

        if let Some(_key) = key {
            let decoding_key = DecodingKey::from_secret(_key.as_ref());
            let validation_token = decode::<Claims>(credential.token(), &decoding_key, &validation);

            if let Ok(value) = &validation_token {
                println!("{:?}", value);
            }
            if validation_token.is_err() {
                return Err((
                    ProblemDetails::unauthorized("Invalid token".to_string()).into(),
                    request,
                ));
            }
        } else {
            return Err((ProblemDetails::server_error().into(), request));
        }

        Ok(request)
    }

    /*
        async fn validate_token(token: &str) -> Result<User, Error> {
        // EJEMPLO: Aquí implementarías tu lógica real de validación
        // Esto podría incluir:
        // - Verificar JWT
        // - Consultar base de datos
        // - Verificar contra Redis/cache
        // - Validar expiración, etc.

        // Simulación de validación (REEMPLAZAR CON TU LÓGICA)
        match token {
            "valid_token_123" => Ok(User {
                id: 1,
                email: "user@example.com".to_string(),
                role: "user".to_string(),
            }),
            "admin_token_456" => Ok(User {
                id: 2,
                email: "admin@example.com".to_string(),
                role: "admin".to_string(),
            }),
            _ => Err(ErrorUnauthorized("Token inválido o expirado")),
        }
    }
        * */
}
