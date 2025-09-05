pub mod file_filter {

    use std::{fmt::format, io::Write};

    use actix_multipart::Multipart;
    use actix_web::{Error, HttpResponse, web};
    use futures_util::TryStreamExt;

    pub async fn load_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
        std::fs::create_dir_all("./static/upload")?;
        let mut is_right_type = false;

        while let Some(mut field) = payload.try_next().await? {
            let content_type = field.content_type();

            if let Some(value) = content_type {
                let type_image_list = [
                    "image/png",
                    "image/jpg",
                    "image/jpeg",
                    "image/gif",
                    "image/webp",
                ];
                let content_type_str = value.essence_str();
                for image in type_image_list {
                    if content_type_str == image {
                        is_right_type = true;
                    };
                }
            }

            if !is_right_type {
                return Ok(HttpResponse::BadRequest()
                    .body("The file does not correspond to the image extension"));
            };

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
