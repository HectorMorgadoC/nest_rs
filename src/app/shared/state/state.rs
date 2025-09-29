pub mod app_state {
    use diesel::PgConnection;
    use diesel::r2d2::{self, ConnectionManager};
    use sqlx::MySqlPool;

    pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

    #[derive(Clone)]
    pub struct AppState {
        pub database_diesel: DBPool,
        pub database_sqlx: MySqlPool,
    }
}

