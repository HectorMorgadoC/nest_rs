pub(crate) mod service {
    use crate::app::{
        module::auth::{
            models::model::{authorization::User as UserGet, dto::User as UserDto},
            repositories::repository::Repository,
        },
        shared::{
            common::http_error::http_error::ProblemDetails,
            contract::contract::{MiddlewareUserContext, MiddlewareUserValidator},
            middleware::authentication::json_web_token::JwtClaim,
        },
    };
    use bcrypt::{DEFAULT_COST, hash, verify};
    use diesel::result::{DatabaseErrorKind, Error::DatabaseError};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use uuid::Uuid;

    #[derive(Clone)]
    pub struct Service {
        repository: Repository,
    }

    impl Service {
        pub fn new(repository: Repository) -> Self {
            Self { repository }
        }

        pub(crate) async fn create(&self, _dto: &UserDto) -> Result<UserDto, ProblemDetails> {
            let mut new_user = _dto.clone();
            let hash = hash(new_user.password.as_bytes(), DEFAULT_COST);

            if let Ok(password) = hash {
                new_user.password = password;
            } else {
                return Err(ProblemDetails::service_unavailable());
            }

            match self.repository.create(&new_user) {
                Ok(_) => Ok(new_user),
                Err(err) => {
                    if let DatabaseError(DatabaseErrorKind::UniqueViolation, _) = err {
                        Err(ProblemDetails::conflict(err.to_string()))
                    } else {
                        println!("[!] Error create user: ( {} )", err);
                        Err(ProblemDetails::service_unavailable())
                    }
                }
            }
        }

        pub(crate) async fn login(&self, user: UserGet) -> Result<String, ProblemDetails> {
            let user = user.clone();

            match self.repository.get_by_email(user.email.clone()) {
                Ok(user_get) => {
                    if !user_get.is_empty()
                        && verify(user.password, &user_get[0].password).unwrap_or(false)
                    {
                        let mut uuid_usize: usize = 0;

                        if let Some(uuid) = user_get[0].id {
                            uuid_usize = uuid_to_usize(&uuid)
                        }
                        let claim: JwtClaim = JwtClaim::new(
                            "http://localhost:3000".to_string(),
                            user_get[0].email.clone(),
                            uuid_usize,
                        );

                        if let Ok(token) = claim.generate_token() {
                            Ok(token)
                        } else {
                            Err(ProblemDetails::service_unavailable())
                        }
                    } else {
                        Err(ProblemDetails::unauthorized(
                            "Invalid credentials".to_string(),
                        ))
                    }
                }
                Err(err) => {
                    if let DatabaseError(DatabaseErrorKind::UniqueViolation, _) = err {
                        Err(ProblemDetails::conflict(err.to_string()))
                    } else {
                        println!("[!] Error get user: ( {} )", err);
                        Err(ProblemDetails::service_unavailable())
                    }
                }
            }
        }
    }

    #[async_trait::async_trait]
    impl MiddlewareUserValidator for Service {
        async fn validate_user_role_for_middleware(
            &self,
            _email: String,
        ) -> Result<MiddlewareUserContext, ProblemDetails> {
            match self.repository.get_by_email(_email) {
                Ok(user) => Ok(MiddlewareUserContext {
                    email: user[0].email.clone(),
                    roles: user[0].roles.clone(),
                }),
                Err(_) => Err(ProblemDetails::not_found()),
            }
        }
    }

    fn uuid_to_usize(id: &Uuid) -> usize {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        hasher.finish() as usize
    }
    /*
    fn hash_password(password: &str, hash: &str) -> Result<String, BcryptError> {
        let hashed = hash(password.as_bytes(), DEFAULT_COST)?;
        Ok(hashed)
    }
    */
}
