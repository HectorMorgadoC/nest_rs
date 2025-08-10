pub(crate) mod route {
    use actix_web::web;
    use super::super::handlers::handler::*;
    
    pub fn configure_router(cfg: &mut web::ServiceConfig) {
        cfg
            .route("/product",web::get().to(get_all_products))
            .route("/product/{id}",web::get().to(get_by_id_product))
            .route("/product/{id}",web::patch().to(update_product))
            .route("/product",web::post().to(create_product))
            .route("/product/{id}",web::delete().to(delete_product));
        }
}
