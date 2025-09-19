pub mod json_web_token {
    use crate::app::{config::env::env::Env, shared::common::error::app_error::AppError};
    use chrono::{Duration, Utc};
    use jsonwebtoken::{Algorithm, EncodingKey, Header, encode, errors::Error};
    use serde::Serialize;

    #[derive(Serialize)]
    struct Claims {
        issuer: String,
        subject: String,
        expiration: usize,
        issued_at: usize,
        user_id: usize,
    }

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

        pub fn generate_token(&self, id: usize) -> Result<String, AppError> {
            let enviroment_variables = Env::init();
            let key = enviroment_variables.get("KEY");
            let token_time: i16 = enviroment_variables
                .get_parsed("TOKEN_TIME")
                .unwrap_or_else(|| 200);
            let header = Header::new(Algorithm::HS512);
            if let Some(value) = key {
                let encoding_key = EncodingKey::from_secret(value.as_ref());
                let expiration =
                    (Utc::now() + Duration::minutes(token_time as i64)).timestamp() as usize;
                let issued_at = Utc::now().timestamp() as usize;
                let claims: Claims = Claims {
                    issuer: self.issuer.clone(),
                    subject: self.subject.clone(),
                    expiration: expiration,
                    issued_at: issued_at,
                    user_id: self.user_id,
                };
                encode(&header, &claims, &encoding_key).map_err(AppError::JwtError)
            } else {
                Err(AppError::JwtSecretNotConfigured)
            }
        }
    }
}
