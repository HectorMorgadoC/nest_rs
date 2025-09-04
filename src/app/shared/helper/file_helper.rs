pub mod file_filter {

    use std::io::Write;

    use actix_multipart::Multipart;
    use actix_web::{Error, HttpResponse, web};
    use futures_util::TryStreamExt;

    pub async fn load_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
        std::fs::create_dir_all("./static/upload")?; // ← Cambio aquí
        println!("Static/upload directory verified");

        while let Some(mut field) = payload.try_next().await.unwrap() {
            if let Some(file_name) = field.content_disposition().expect("REASON").get_filename() {
                let file_path = format!("./static/upload/{file_name}");
                println!("Saving: {}", file_path.clone());

                let mut archivo = web::block(move || std::fs::File::create(&file_path)).await??;

                while let Some(chunk) = field.try_next().await? {
                    archivo =
                        web::block(move || archivo.write_all(&chunk).map(|_| archivo)).await??;
                }

                println!("Saved file");
            }
        }

        Ok(HttpResponse::Ok().body("File saved successfully"))
    }
}
