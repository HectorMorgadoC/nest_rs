pub(crate) mod route {
    use super::super::handlers::handler::*;
    use actix_web::web::{self, ServiceConfig};

    pub fn configure_routes(cfg: &mut ServiceConfig) {
        cfg.service(
            web::scope("/team")
                .route("", web::post().to(create_team))
                .route("", web::get().to(get_all_team))
                .route("/{id}", web::get().to(get_find_by_name_team))
                .route("/{id}", web::patch().to(update_team))
                .route("", web::delete().to(delete_team)),
        );
    }
}
