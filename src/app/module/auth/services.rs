pub(crate) mod service {
    use bcrypt::{BcryptError, DEFAULT_COST, hash};
    use diesel::result::{DatabaseErrorKind, Error::DatabaseError};

    use crate::app::{
        module::auth::{
            models::model::{authorization::User as USER, dto::User as DTO},
            repositories::repository::Repository,
        },
        shared::common::http_error::http_error::ProblemDetails,
    };

    pub struct Service {
        repository: Repository,
    }

    impl Service {
        pub fn new(repository: Repository) -> Self {
            Self { repository }
        }

        pub(crate) async fn create(&self, _dto: &DTO) -> Result<DTO, ProblemDetails> {
            let mut new_user = _dto.clone();
            let new_password = hash_password(&new_user.password);

            if let Ok(password) = new_password {
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

        pub(crate) async fn get(&self, user: USER) -> Result<DTO, ProblemDetails> {}
    }

    fn hash_password(password: &str) -> Result<String, BcryptError> {
        let hashed = hash(password.as_bytes(), DEFAULT_COST)?;
        Ok(hashed)
    }
}
