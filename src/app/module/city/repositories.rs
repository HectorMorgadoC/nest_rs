pub mod repository {
    use super::super::models::model::City;

    use crate::app::shared::common::pagination_dto::pagination_dto::Pagination;
    use sqlx::{Error as error, MySqlPool};

    pub(crate) struct Repository {
        pool: MySqlPool,
    }

    impl Repository {
        pub fn new(pool: MySqlPool) -> Self {
            Self { pool }
        }

        pub async fn create_city(&self, dto: City) -> Result<City, error> {
            sqlx::query_as::<_, City>(
                "INSERT INTO product (id, name, ip)
                    VALUES ($1, $2, $3)
                    RETURNING id, name, ip",
            )
            .bind(&dto.id)
            .bind(&dto.name)
            .bind(dto.ip)
            .fetch_one(&self.pool)
            .await
        }

        pub async fn get_all_city(&self) -> Result<Vec<City>, error> {
            sqlx::query_as::<_, City>("SELECT id, name, ip FROM city")
                .fetch_all(&self.pool)
                .await
        }

        pub async fn get_pagination_city(&self, filter: Pagination) -> Result<Vec<City>, error> {
            sqlx::query_as::<_, City>("SELECT id, name, ip FROM city LIMIT $1 OFFSET $2")
                .bind(filter.limit)
                .bind(filter.offset)
                .fetch_all(&self.pool)
                .await
        }

        pub async fn get_city_by_id(&self, id: i32) -> Result<Option<City>, error> {
            sqlx::query_as::<_, City>("SELECT id, name, ip FROM city WHERE id = $1")
                .bind(id)
                .fetch_optional(&self.pool)
                .await
        }

        pub async fn update_city(&self, id: i32, dto: City) -> Result<City, error> {
            sqlx::query_as::<_, City>(
                "UPDATE city
                    SET name = $1, ip = $2 
                    WHERE id = $3 
                    RETURNING id, name, ip",
            )
            .bind(&dto.name)
            .bind(dto.ip)
            .bind(id)
            .fetch_one(&self.pool)
            .await
        }

        pub async fn delete_city(&self, id: i32) -> Result<u64, error> {
            let result = sqlx::query("DELETE FROM city WHERE id = $1")
                .bind(id)
                .execute(&self.pool)
                .await
                .unwrap();

            Ok(result.rows_affected())
        }
    }
}
