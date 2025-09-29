pub(crate) mod service {

    use crate::app::shared::common::http_error::http_error::ProblemDetails;
    use diesel::result::{DatabaseErrorKind, Error::DatabaseError};
    use uuid::Uuid;

    use super::super::{models::model::dto::Product, repositories::repository::Repository};

    pub(crate) struct Service {
        repository: Repository,
    }

    impl Service {
        pub fn new(repository: Repository) -> Self {
            Self { repository }
        }

        pub(crate) async fn get(&self) -> Result<Vec<Product>, ProblemDetails> {
            // add code here
            match self.repository.get_all() {
                Ok(products) => {
                    if !products.is_empty() {
                        Ok(products)
                    } else {
                        Err(ProblemDetails::not_found())
                    }
                }
                Err(err) => {
                    println!("Error request product: {err}");
                    Err(ProblemDetails::server_error())
                }
            }
        }

        pub(crate) async fn get_by_id(&self, _id: &Uuid) -> Result<Vec<Product>, ProblemDetails> {
            match self.repository.get_by_id(&_id) {
                Ok(products) => {
                    let product_size = products.len();
                    if product_size > 0 {
                        Ok(products)
                    } else {
                        Err(ProblemDetails::not_found())
                    }
                }
                Err(err) => {
                    println!("Error request product: {err}");
                    Err(ProblemDetails::server_error())
                }
            }
        }

        pub(crate) async fn create(&self, _dto: &Product) -> Result<Product, ProblemDetails> {
            let updated_product = _dto.clone();
            match self.repository.create(_dto) {
                Ok(_) => Ok(updated_product),
                Err(err) => {
                    if let DatabaseError(DatabaseErrorKind::UniqueViolation, _) = err {
                        Err(ProblemDetails::conflict(err.to_string()))
                    } else {
                        Err(ProblemDetails::service_unavailable())
                    }
                }
            }
        }

        pub(crate) async fn update(
            &self,
            _dto: &Product,
            _id: &Uuid,
        ) -> Result<String, ProblemDetails> {
            match self.repository.update(_dto, _id) {
                Ok(1) => Ok(format!("Updated product: {}", _id.clone())),
                Ok(0) => Err(ProblemDetails::not_found()),
                Ok(_) => Err(ProblemDetails::conflict(
                    "Update affected rows; expected 1".to_string(),
                )),
                Err(err) => {
                    if let DatabaseError(DatabaseErrorKind::UniqueViolation, _)
                    | DatabaseError(DatabaseErrorKind::ForeignKeyViolation, _) = err
                    {
                        Err(ProblemDetails::conflict(err.to_string()))
                    } else {
                        Err(ProblemDetails::service_unavailable())
                    }
                }
            }
        }

        pub(crate) async fn delete(&self, _id: &Uuid) -> Result<(), ProblemDetails> {
            let id = _id.clone();
            match self.repository.delete_product_image(&id) {
                Ok(0) => println!("There is no record that this id: {}", &id),
                Ok(_) => println!("{} Record deleted", &id),
                Err(err) => {
                    if let DatabaseError(DatabaseErrorKind::UniqueViolation, _)
                    | DatabaseError(DatabaseErrorKind::ForeignKeyViolation, _) = err
                    {
                        return Err(ProblemDetails::conflict(err.to_string()));
                    } else {
                        return Err(ProblemDetails::service_unavailable());
                    }
                }
            }

            match self.repository.delete(_id) {
                Ok(0) => Err(ProblemDetails::not_found()),
                Ok(_) => Ok(()),
                Err(err) => {
                    if let DatabaseError(DatabaseErrorKind::UniqueViolation, _)
                    | DatabaseError(DatabaseErrorKind::ForeignKeyViolation, _) = err
                    {
                        Err(ProblemDetails::conflict(err.to_string()))
                    } else {
                        Err(ProblemDetails::service_unavailable())
                    }
                }
            }
        }
    }
}
