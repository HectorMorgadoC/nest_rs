pub(crate) mod handler {
    use actix_web::{web, HttpResponse,Responder};
    use uuid::Uuid;
    use crate::app::{module::product::services::service::Service, shared::common::message_detail::message_detail::ProblemDetails};
    

    use super::super::models::model::dto::Product;
    
    pub async fn get_all_products(service: web::Data<Service>) -> Result<impl Responder,ProblemDetails> {
        println!("Peticion a GET: /product");
        match service.get_all().await {
            Ok(products) => Ok(HttpResponse::Ok().json(products)),
            Err(err) => Err(err)
        }
    }
    
    pub async fn get_by_id_product(id: web::Path<Uuid>) -> impl Responder {
        HttpResponse::Ok().finish()
    }
    
    pub async fn create_product(dto: web::Data<Product>) -> impl Responder {
        HttpResponse::Ok().finish()
    }
    pub async fn update_product(dto: web::Data<Product>) -> impl Responder {
        HttpResponse::Ok().finish()
    }
    
    pub async fn delete_product(id: web::Path<Uuid>) -> impl Responder {
        HttpResponse::Ok().finish()
    }
}
