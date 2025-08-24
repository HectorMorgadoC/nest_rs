pub(crate) mod handler {
    use super::super::models::model::dto::ValidatedProduct;
    use crate::app::module::product::models::model::dto::Product;
    use crate::app::module::product::services::service::Service as Services;
    use crate::app::shared::common::{
        http_error::http_error::HttpResult, message_detail::message_detail::Details,
    };
    use crate::app::shared::{
        api::api_request::api_request::DataRequest,
        validation::validation::validation::ValidatedRequest,
    };
    use actix_web::{HttpResponse, web};
    use uuid::Uuid;

    pub async fn get_all_products(service: web::Data<Services>) -> HttpResult<HttpResponse> {
        println!("GET: /product");
        let products = service.get_all().await?;
        //match service.get_all().await {
        //    Ok(products) => HttpResponse::Ok().json(products),
        //    Err(err) => err.error_response()
        //}
        Ok(HttpResponse::Ok().json(products))
    }

    pub async fn get_by_id_product(
        service: web::Data<Services>,
        _id: web::Path<Uuid>,
    ) -> HttpResult<HttpResponse> {
        //let id = _id.into_inner();
        println!("GET: /product/{}", &_id);
        let product = service.get_by_id(&_id).await?;

        //match service.get_by_id(&_id).await {
        //    Ok(products) => Ok(HttpResponse::Ok().json(products)),
        //    Err(err) => Err(err)
        //}
        Ok(HttpResponse::Ok().json(product))
    }

    //pub async fn create_product(service: web::Data<Services>,ValidatedProduct(dto): ValidatedProduct) -> HttpResult<HttpResponse> {
    //    println!("POST: /product");
    //    let product_record = service.create(&dto).await?;
    //    Ok(HttpResponse::Ok().json(product_record))
    //}

    pub async fn create_product(
        service: web::Data<Services>,
        ValidatedRequest(dto): ValidatedRequest<DataRequest<Product>>,
    ) -> HttpResponse {
        println!("POST: /product");
        println!("{:?}", dto);
        HttpResponse::Ok().finish()
    }

    pub async fn update_product(
        service: web::Data<Services>,
        ValidatedProduct(dto): ValidatedProduct,
        _id: web::Path<Uuid>,
    ) -> HttpResult<HttpResponse> {
        println!("PUT: /product");
        let id = _id.into_inner();
        let modified_product = service.update(&dto, &id).await?;

        Ok(HttpResponse::Ok().json(Details::new(
            "Update product".to_string(),
            200,
            modified_product,
            format!("/product/{}", id),
        )))
    }

    pub async fn delete_product(
        service: web::Data<Services>,
        _id: web::Path<Uuid>,
    ) -> HttpResult<HttpResponse> {
        service.delete(&_id).await?;

        Ok(HttpResponse::NoContent().finish())
    }
}
