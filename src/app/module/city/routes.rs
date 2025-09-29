pub mod route {
    use super::super::handlers::handler::*;
    use crate::app::shared::middleware::{
        authentication::json_web_token::validator, validate::validate_role::role_validator,
    };
    use actix_web::web;
    use actix_web_httpauth::middleware::HttpAuthentication;

    pub fn configure_router(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/city")
                .wrap(HttpAuthentication::bearer(validator))
                .wrap(HttpAuthentication::bearer(role_validator(vec![
                    "admin".to_string(),
                ])))
                .route("", web::get().to(get_all_city))
                .route("/{id}", web::get().to(get_by_id_city))
                .route("/{id}", web::put().to(update_city))
                .route("", web::post().to(create_city))
                .route("/{id}", web::delete().to(delete_city)),
        );
    }
}
