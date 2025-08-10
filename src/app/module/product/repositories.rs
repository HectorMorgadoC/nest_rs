pub(crate) mod repository {

        use diesel::result::Error as error;
        use super::super::models::model::{dto::Product,schemac::product::dsl::*};
        use crate::app::shared::database::diesel::diesel::DBPool;
        use diesel::prelude::*; // sin este impor
        use uuid::Uuid;
        
        
        pub struct Repository {
            connection: DBPool
        }
    
        impl Repository {
            
            pub fn new(connection: DBPool) -> Self {
                Self { connection }
            }
    
            
            pub fn get_all(&self) -> Result<Vec<Product>,error> {
                let mut connection = self.connection.get().unwrap();
                let result = product
                    .select(Product::as_select())
                    .load(&mut connection);

                    result
            }
            
            
            pub fn get_by_id(&mut self,_id: Uuid) -> Result<Vec<Product>,error> {
                let mut connection = self.connection.get().unwrap();  
                                                
                      let result = product
                            .filter(id.eq(_id))
                            .select(Product::as_select())
                            .load(&mut connection);

                    result
            }
                    
            pub fn create(&mut self,dto: Product) -> Result<usize,error> {
                let mut connection = self.connection.get().unwrap();
                diesel::insert_into(product)
                    .values(&dto)
                    .execute(&mut connection)
            }
    
            pub fn update(&mut self,_dto: Product,_id: Uuid) -> Result<usize,error> {
                let mut connection = self.connection.get().unwrap();
                diesel::update(product)
                .filter(id.eq(_id))
                .set(_dto)
                .execute(&mut connection)
            }
            
            pub fn delete(&mut self,_id: Uuid) -> Result<usize,error> {
                let mut connection = self.connection.get().unwrap();
                diesel::delete(product)
                    .filter(id.eq(_id))
                    .execute(&mut connection)
            }
        }
}
