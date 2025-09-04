pub(crate) mod route {
    use super::super::handlers::handler::*;
    use actix_web::web::{self, ServiceConfig};

    pub fn configure_router(cfg: &mut ServiceConfig) {
        cfg.route("/upload", web::get().to(upload_file_handler));
    }
}
