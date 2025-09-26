pub(crate) mod route {
    use super::super::handlers::handler::*;
    use crate::app::shared::middleware::{
        authentication::json_web_token::validator, validate::validate_role::validate_role_admin,
    };
    use actix_web::web;
    use actix_web_httpauth::middleware::HttpAuthentication;

    pub fn configure_router(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/product")
                .wrap(HttpAuthentication::bearer(validator))
                .wrap(HttpAuthentication::bearer(validate_role_admin))
                .route("", web::get().to(get_all_products))
                .route("/{id}", web::get().to(get_by_id_product))
                .route("/{id}", web::put().to(update_product))
                .route("", web::post().to(create_product))
                .route("/{id}", web::delete().to(delete_product)),
        );
    }
}
