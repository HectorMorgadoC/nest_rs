pub mod file_filter {

    use actix_multipart::Multipart;
    use actix_web::{Error, HttpResponse, web};
    use futures_util::TryStreamExt;
    use std::{fs::File, io::Write};
    use uuid::Uuid;

    pub async fn load_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
        std::fs::create_dir_all("./static/upload")?;
        let mut is_right_type = false;
        let mut url_response_image = String::new();

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
                let new_file_name = file_name_helper(file_name);
                url_response_image.push_str(&new_file_name);
                let file_path = format!("./static/upload/{}", &new_file_name);
                println!("Saving: {}", file_path.clone());

                let mut file = web::block(move || std::fs::File::create(&file_path)).await??;

                while let Some(chunk) = field.try_next().await? {
                    file = web::block(move || file.write_all(&chunk).map(|_| file)).await??;
                }

                println!("Saved file");
            }
        }

        Ok(HttpResponse::Ok().body(format!(
            "http://localhost:3000/static/upload/{url_response_image}"
        )))
    }

    fn file_name_helper(file_name: &str) -> String {
        let mut uuid = Uuid::new_v4();
        let extension: Vec<&str> = file_name.split(".").collect();

        while true {
            let uuid_in_data = File::open(format!(
                "./static/upload/{uuid}.{:?}",
                extension.last().map_or("---", |v| v)
            ));

            if uuid_in_data.is_ok() {
                uuid = Uuid::new_v4();
            } else {
                break;
            }
        }

        format!("{}.{}", uuid, extension.last().unwrap())
    }
}
