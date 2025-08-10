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

pub fn configure_providers(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::Data::new(service_init));
}

fn service_init(connection: web::Data<AppState>) -> Service {
    let repository: Repository = Repository::new(connection.clone().db.clone());
    Service::new(repository)
}      
