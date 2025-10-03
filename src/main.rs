mod app;
mod schema;
use crate::app::config::env::env::Env;
use crate::app::{
    module::{configure_providers, configure_routes},
    shared::state::state::app_state::AppState,
};
use actix_files::Files;
use actix_web::{App, HttpServer, web};
use app::shared::database::{
    diesel::diesel::connection as connection_diesel,
    mongo::mongo_db::connection as connection_mongo,
    sqlx_mysql::sqlx::connection as connection_sqlx,
};

use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let var_env = Env::init();
    let env_port: u16 = var_env.get_parsed("PORT").unwrap_or_else(|| 3000);
    let env_address: &str = &var_env.get_or("ADDRESS", "127.0.0.1");
    let url_database_diesel = var_env.get("DATABASE_URL").unwrap();
    let url_database_sqlx = var_env.get("DATABASE_URL_MYSQL").unwrap();
    let url_database_mongo = var_env.get("MONGO_URI").unwrap();
    let client_mongo = "client_db".to_string();
    let database_connection_diesel = connection_diesel(url_database_diesel);
    let database_connection_sqlx = connection_sqlx(url_database_sqlx).await;
    let database_connection_mongo =
        connection_mongo(url_database_mongo.to_string(), &client_mongo).await;
    let app_data = web::Data::new(AppState {
        database_diesel: database_connection_diesel,
        database_sqlx: database_connection_sqlx,
        database_mongo: database_connection_mongo,
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(Files::new("upload", "./static/upload").show_files_listing())
            .configure(configure_routes)
            .configure(|cfg| configure_providers(cfg, app_data.clone()))
    });

    println!("server running port: {env_port}");

    server.bind((env_address, env_port)).unwrap().run().await
}
