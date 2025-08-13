pub(crate) mod service {    
    use std::io::Error as error;
    use crate::app::shared::common::message_detail::message_detail::ProblemDetails;
    use uuid::Uuid;
    use validator::ValidateLength;

    use super::super::{
        repositories::repository::Repository,
        models::model::dto::Product
    };
    
     
    pub(crate) struct Service {
        repository: Repository
    }
    
    impl Service {
            
        pub fn new(repository: Repository) -> Self {
            Self { repository }
        }
    
        pub(crate) async fn get_all(&self) -> Result<Vec<Product>,ProblemDetails> {
            match self.repository.get_all() {
                Ok(products) => {
                    let product_size = products.len(); 
                    if product_size > 0 {
                        Ok(products)
                    } else {
                        Err(ProblemDetails::not_found("/product".to_string()))
                    }
                },
                Err(err) => {
                    println!("Error request product: {err}");
                    Err(ProblemDetails::server_error("/product".to_string()))
                }
            }   
        }
        
        pub(crate) async fn get_by_id(&mut self, id: Uuid) -> Result<Vec<Product>,ProblemDetails> {
            match self.repository.get_by_id(id){
                Ok(products) => {
                    let product_size = products.len(); 
                    if product_size > 0 {
                        Ok(products)
                    } else {
                        Err(ProblemDetails::not_found("/product/id".to_string()))
                    }
                },
                Err(err) => {
                    println!("Error request product: {err}");
                    Err(ProblemDetails::server_error("/product/id".to_string()))
                }
            }   
        }
    
        pub(crate) async fn create(&self, dto: Product) -> Result<Product,ProblemDetails> {
                Ok(dto)
        }
    
        pub(crate) async  fn update(&self, dto: Product, id: String) -> Result<Product,error> {
            let product: Product = Product::default();
            Ok(product)
        }
            
        pub(crate) async fn delete(&self, id: String) -> Result<(),error> {
            let product: Product = Product::default();
            Ok(())
        }
                    
    }
}
