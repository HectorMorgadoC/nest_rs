use crate::app::shared::{
    contract::contract::MiddlewareUserValidator, state::state::app_state::AppState,
};
use actix_web::web;
use std::sync::Arc;

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
    let repository: Repository = Repository::new(app_state.database_diesel.clone());
    let service: Service = Service::new(repository);
    cfg.app_data(web::Data::new(service.clone()));
    configure_middleware_dependencies(cfg, service);
}

fn configure_middleware_dependencies(cfg: &mut web::ServiceConfig, service: Service) {
    let middleware_validator: Arc<dyn MiddlewareUserValidator> = Arc::new(service);
    cfg.app_data(web::Data::new(middleware_validator));
}
