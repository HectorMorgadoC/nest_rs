use actix_web::web;
use crate::app::shared::state::state::state::AppState;

use super::super::module::product::{
        routes::route,
        services::service::Service,
        repositories::repository::Repository
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
