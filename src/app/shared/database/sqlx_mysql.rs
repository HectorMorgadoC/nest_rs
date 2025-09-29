pub mod sqlx {
    use sqlx::{MySqlPool, mysql::MySqlPoolOptions};

    pub async fn connection(url: &str) -> MySqlPool {
        MySqlPoolOptions::new()
            .max_connections(20)
            .connect(url)
            .await
            .unwrap_or_else(|_| panic!("Error creating DB pool for {url}"))
    }
}
