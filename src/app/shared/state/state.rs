pub mod state {
    use diesel::r2d2::{self, ConnectionManager};
    use diesel::PgConnection;
    
    pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;
    
    #[derive(Clone)]
    pub struct AppState {
        pub db: DBPool,
    }
}