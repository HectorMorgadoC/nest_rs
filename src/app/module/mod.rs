use actix_web::web::{ServiceConfig, Data};
use crate::app::shared::state::state::state::AppState;
mod product;

pub fn configure_providers(cfg : &mut ServiceConfig, app_state: Data<AppState>) {
    cfg.configure(|c| product::configure_providers(c, app_state));
}

pub fn configure_routes(cfg : &mut ServiceConfig) {
    cfg.configure(product::configure_routes);
}
