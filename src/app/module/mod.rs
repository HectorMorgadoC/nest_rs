use actix_web::web::ServiceConfig;
mod product;

pub fn configure_providers(cfg : &mut ServiceConfig) {
    cfg.configure(product::configure_providers);
}

pub fn configure_routes(cfg : &mut ServiceConfig) {
    cfg.configure(product::configure_routes);
}
