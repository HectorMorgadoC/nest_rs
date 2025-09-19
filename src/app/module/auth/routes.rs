pub(crate) mod route {
    use actix_web::web;

    use super::super::handlers::handler::*;

    pub fn configure_router(cfg: &mut web::ServiceConfig) {
        cfg.route("/auth", web::post().to(create_user));
    }
}
