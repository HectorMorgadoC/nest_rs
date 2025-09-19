use crate::app::shared::state::state::state::AppState;
use actix_web::web;

use super::super::module::auth::{
    repositories::repository::Repository, routes::route, services::service::Service,
};

mod handlers;
mod models;
mod repositories;
mod routes;
mod services;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(route::configure_router);
}

pub fn configure_providers(cfg: &mut web::ServiceConfig, app_state: web::Data<AppState>) {
    let repository: Repository = Repository::new(app_state.db.clone());
    let service: Service = Service::new(repository);
    cfg.app_data(web::Data::new(service));
}
