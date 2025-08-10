pub mod diesel {
    use diesel::{pg::PgConnection, r2d2::{self,ConnectionManager}};
    
    pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;
    
    pub fn connection(url: &str) -> DBPool {
        
        let manager = ConnectionManager::<PgConnection>::new(url);
        r2d2::Pool::builder()
            .build(manager)
            .unwrap_or_else(|_| panic!("Error creating DB pool for {}", url))   
    }
}