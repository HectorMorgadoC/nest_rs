pub(crate) mod model {
    
    pub mod schemac {
        diesel::table! {
            product (id) {
                id -> Nullable<Uuid>,
                title -> Varchar,
                price -> Float,
                description -> Varchar,
                slug -> Varchar,
                stock -> Integer,
                images -> Array<Varchar>,
                gender -> Varchar
            }
        }
        
        diesel::table! {
            product_image (id) {
                id -> Nullable<Uuid>,
                product_id -> Uuid,
                url -> Varchar,
            }
        }
        
        diesel::joinable!(product_image -> product(product_id));
        diesel::allow_tables_to_appear_in_same_query!(
            product,
            product_image
        );
    }

    pub mod dto {
        use diesel::prelude::{AsChangeset, Insertable, Queryable, Selectable};
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
        use super::schemac::product;

        #[derive(Queryable, Selectable,Debug,Serialize,Deserialize,Validate,Insertable,AsChangeset,Default,)]
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
            pub images: Vec<String>,
            #[validate(length(min = 3,max = 50))]
            gender: String
    
        }
        
        #[derive(Queryable, Selectable,Debug,Serialize,Deserialize,Validate,Insertable,AsChangeset,Default)]
        #[diesel(table_name = super::schemac::product_image)]
        #[diesel(check_for_backend(diesel::pg::Pg))]
        pub struct ProductImage {
            pub id: Option<Uuid>,
            pub product_id: Uuid,
            #[validate(length(min = 8))]
            pub url: String,
        }

        #[derive(Debug)]
        pub struct ValidatedProduct(Product);
        use crate::app::shared::common::message_detail::message_detail::ProblemDetails;
        
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
                            ProblemDetails::bad_request(uri.clone(), e.to_string())
                        })?;

                    let product = json_product.into_inner();


                     // Validar con validator
                    if let Err(validation_errors) = product.validate() {
                            return Err(ProblemDetails::unprocesable_entity(uri, validation_errors.to_string()));
                    }
                    
                            Ok(ValidatedProduct(product))
                                                           // Validar usando validator
                })
            }
        }


        }
      
    }



