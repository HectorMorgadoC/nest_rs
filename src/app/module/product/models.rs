pub(crate) mod model {

    pub mod dto {
        
        use diesel::prelude::*;
        use serde::{Deserialize, Serialize};
        use uuid::Uuid;
        use validator::Validate;
        use actix_web::{
            web::Json,
            HttpRequest,
            FromRequest
        };
        use std::pin::Pin;
        use std::future::Future;
        use crate::schema::{product,product_image};
        //use crate::schema::product_image;

        #[derive(Queryable, Selectable, Debug,Clone,Serialize, Deserialize, Validate, Insertable, AsChangeset, Default)]
        #[diesel(table_name = product )]
        #[diesel(check_for_backend(diesel::pg::Pg))]
        pub struct Product {
            pub id: Option<Uuid>,
            #[validate(length(min = 3,max = 50))]
            pub title: String,
            #[validate(length(min = 3,max = 200))]
            pub description: String,
            #[validate(length(min = 3,max = 50))]
            pub slug: String,
            #[validate(range(min = 0.0))]
            pub price: f32,
            #[validate(range(min = 0))]
            pub stock: i32,
            #[validate(length(min = 3,max = 50))]
            pub gender: String
    
        }
        
        #[derive(Queryable, Selectable,Debug,Serialize,Deserialize,Validate,Insertable,AsChangeset,Default)]
        #[diesel(table_name = product_image)]
        #[diesel(check_for_backend(diesel::pg::Pg))]
        //#[diesel(belongs_to(Product, foreign_key = product_id))]
        pub struct ProductImage {
            pub id: Option<Uuid>,
            pub product_id: Uuid,
            #[validate(length(min = 8))]
            pub url: String,
        }
 

        #[derive(Debug)]
        pub struct ValidatedProduct(pub Product);
        use crate::app::shared::common::http_error::http_error::ProblemDetails;
        
        impl FromRequest for ValidatedProduct {
            type Error = ProblemDetails;
            type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;


            fn from_request(req: &HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
                let json_future = Json::<Product>::from_request(req, payload);
                let uri = req.uri().to_string();
                
                Box::pin(async move {
                    let json_product = json_future
                        .await
                        .map_err(|e| {
                            ProblemDetails::bad_request(e.to_string())
                        });

                    let product = json_product.expect("REASON").into_inner();


                     // Validar con validator
                    if let Err(validation_errors) = product.validate() {
                            Err(ProblemDetails::unprocesable_entity(validation_errors.to_string()))
                    } else {
                            Ok(ValidatedProduct(product))
                    }
                    
                })
            }
        }


        }
      
    }



