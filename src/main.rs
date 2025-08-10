mod app;
use actix_web::{App,HttpServer,web};
use std::io;
use app::shared::database::diesel::diesel::connection;
use crate::app::{module::{configure_providers, configure_routes}, shared::state::state::state::AppState};
use crate::app::config::env::env::Env;

#[actix_web::main]
async fn main() -> io::Result<()> {
    
    let var_env = Env::init();
    let env_port: u16 = var_env.get_parsed("PORT").unwrap_or_else(|| 3000);
    let env_address: &str = &var_env.get_or("ADDRESS", "127.0.0.1");
    let url_database = var_env.get("DATABASE_URL").unwrap();
    let database_connection = connection(&url_database);
    let app_data = web::Data::new(AppState {
        db: database_connection,
    });
    let server = HttpServer::new(move || {
        
        App::new()
            .app_data(app_data.clone())
            .configure(configure_routes)
            .configure(configure_providers)
    });

    println!("server running port: {env_port}");

    server.bind((env_address,env_port)).unwrap()
    .run()
    .await
}
