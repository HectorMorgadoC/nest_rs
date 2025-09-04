use crate::app::shared::state::state::state::AppState;
use actix_web::web::{Data, ServiceConfig};
mod file;
mod product;

pub fn configure_providers(cfg: &mut ServiceConfig, app_state: Data<AppState>) {
    cfg.configure(|c| product::configure_providers(c, app_state));
}

pub fn configure_routes(cfg: &mut ServiceConfig) {
    cfg.configure(product::configure_routes)
        .configure(file::configure_routes);
}
