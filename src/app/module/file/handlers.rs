pub mod handler {
    use crate::app::shared::helper::file_helper::file_filter;
    use actix_multipart::Multipart;
    use actix_web::{HttpResponse, Responder, body};

    pub(crate) async fn upload_file_handler(payload: Multipart) -> impl Responder {
        println!("GET: /upload");

        // Llamar directamente a tu función
        match file_filter::load_file(payload).await {
            Ok(value) => value,
            Err(err) => HttpResponse::Ok().body(format!("{}", err)),
        }
    }
}
