use crate::app::shared::state::state::app_state::AppState;
use actix_web::web;

mod handlers;
mod models;
mod repositories;
mod routes;
mod services;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(routes::route::configure_routes);
}

pub fn configure_providers(cfg: &mut web::ServiceConfig, app_state: web::Data<AppState>) {
    let repository = repositories::repository::Repository::new(app_state.database_mongo.clone());
    let service = services::service::Service::new(repository);
    cfg.app_data(web::Data::new(service));
}
