pub mod service {
    use super::super::models::model::City;
    use super::super::repositories::repository::Repository;
    use crate::app::shared::common::http_error::http_error::ProblemDetails;
    use crate::app::shared::common::pagination_dto::pagination_dto::Pagination;

    pub struct Service {
        repository: Repository,
    }

    impl Service {
        pub fn new(repository: Repository) -> Self {
            Self { repository }
        }

        pub async fn get(&self) -> Result<Vec<City>, ProblemDetails> {
            let cities = self
                .repository
                .get_all_city()
                .await
                .map_err(handle_database_error)?;

            if !cities.is_empty() {
                Ok(cities)
            } else {
                Err(ProblemDetails::not_found())
            }
        }

        pub async fn get_pagination(&self, query: Pagination) -> Result<Vec<City>, ProblemDetails> {
            let cities = self
                .repository
                .get_pagination_city(query)
                .await
                .map_err(handle_database_error)?;

            if !cities.is_empty() {
                Ok(cities)
            } else {
                Err(ProblemDetails::not_found())
            }
        }

        pub async fn get_find_by_id(&self, id: i32) -> Result<City, ProblemDetails> {
            let city = self
                .repository
                .get_city_by_id(id)
                .await
                .map_err(handle_database_error)?;

            if let Some(_city) = city {
                Ok(_city)
            } else {
                Err(ProblemDetails::not_found())
            }
        }

        pub async fn create(&self, dto: City) -> Result<City, ProblemDetails> {
            let city = self
                .repository
                .create_city(dto)
                .await
                .map_err(handle_database_error)?;

            Ok(city)
        }

        pub async fn update(&self, dto: City, id: i32) -> Result<City, ProblemDetails> {
            let city = self
                .repository
                .update_city(id, dto)
                .await
                .map_err(handle_database_error)?;

            Ok(city)
        }

        pub async fn delete(&self, id: i32) -> Result<(), ProblemDetails> {
            let city = self
                .repository
                .delete_city(id)
                .await
                .map_err(handle_database_error)?;

            if city == 0 {
                Err(ProblemDetails::not_found())
            } else {
                Ok(())
            }
        }
    }

    pub fn handle_database_error(err: sqlx::Error) -> ProblemDetails {
        match err {
            sqlx::Error::Database(db_err) => {
                let code = db_err.code().map(|c| c.to_string()).unwrap_or_default();
                let message = db_err.message().to_string();

                println!("Database error [{}]: {}", code, message);

                match code.as_ref() {
                    // Integridad y Constraints
                    "23000" => ProblemDetails::conflict(message),
                    "23502" => ProblemDetails::bad_request(message),
                    "23503" => ProblemDetails::conflict(message),
                    "23505" => ProblemDetails::conflict(message),
                    "23514" => ProblemDetails::bad_request(message),

                    // Datos inválidos
                    "22001" => ProblemDetails::bad_request("Data too long for field".to_string()),
                    "22003" => {
                        ProblemDetails::bad_request("Numeric value out of range".to_string())
                    }
                    "22007" => ProblemDetails::bad_request("Invalid datetime format".to_string()),
                    "22008" => ProblemDetails::bad_request("Datetime out of range".to_string()),
                    "22012" => ProblemDetails::bad_request("Division by zero".to_string()),
                    "22018" => ProblemDetails::bad_request("Invalid type conversion".to_string()),

                    // Sintaxis y objetos
                    "42000" => ProblemDetails::bad_request("SQL syntax error".to_string()),
                    "42S02" => ProblemDetails::server_error(),
                    "42S22" => ProblemDetails::server_error(),
                    "42703" => ProblemDetails::server_error(),
                    "42883" => ProblemDetails::server_error(),

                    // Conexión y transacciones
                    "08001" | "08003" | "08006" => ProblemDetails::service_unavailable(),
                    "25000" => ProblemDetails::service_unavailable(),
                    "40001" | "40P01" => ProblemDetails::service_unavailable(),

                    // Privilegios
                    "28000" | "28P01" => ProblemDetails::service_unavailable(),
                    "42501" => ProblemDetails::service_unavailable(),

                    // Recursos
                    "53000" | "53100" | "53200" => ProblemDetails::service_unavailable(),
                    "53300" => ProblemDetails::service_unavailable(),
                    "57014" => ProblemDetails::service_unavailable(),
                    "HYT00" | "HYT01" => ProblemDetails::service_unavailable(),

                    // Default
                    _ => ProblemDetails::server_error(),
                }
            }

            sqlx::Error::RowNotFound => ProblemDetails::not_found(),

            sqlx::Error::ColumnNotFound(col) => ProblemDetails::server_error(),

            sqlx::Error::PoolTimedOut => ProblemDetails::service_unavailable(),

            sqlx::Error::PoolClosed => ProblemDetails::service_unavailable(),

            _ => {
                println!("Unexpected database error: {}", err);
                ProblemDetails::server_error()
            }
        }
    }
}
