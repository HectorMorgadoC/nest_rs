use crate::app::shared::state::state::app_state::AppState;
use actix_web::web::{Data, ServiceConfig};
mod auth;
mod city;
mod file;
mod product;
mod team;

pub fn configure_providers(cfg: &mut ServiceConfig, app_state: Data<AppState>) {
    cfg.configure(|c| product::configure_providers(c, app_state.clone()))
        .configure(|c| auth::configure_providers(c, app_state.clone()))
        .configure(|c| city::configure_providers(c, app_state.clone()))
        .configure(|c| team::configure_providers(c, app_state.clone()));
}

pub fn configure_routes(cfg: &mut ServiceConfig) {
    cfg.configure(product::configure_routes)
        .configure(file::configure_routes)
        .configure(auth::configure_routes)
        .configure(city::configure_routes)
        .configure(team::configure_routes);
}
