pub(crate) mod repository {
  
        use diesel::associations::HasTable; 
        use diesel::result::Error as error;
        use crate::schema::product_image::product_id;
        use crate::schema::{product::dsl::*,product_image};
        use crate::schema::product_image::dsl::product_image as other_product_image;
        use crate::app::shared::database::diesel::diesel::DBPool;
        use diesel::prelude::*; 
        use uuid::Uuid;
        use super::super::models::model::dto::{Product,ProductImage};
        use std::collections::HashMap;
        
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
            
            
            pub fn get_by_id(&self,_id: &Uuid) -> Result<Vec<Product>,error> {
                let mut connection = self.connection.get().unwrap();  
                                                
                      let result = product
                            .filter(id.eq(_id))
                            .select(Product::as_select())
                            .load(&mut connection);

                    result
            }
                    
            pub fn create(&self,_dto: &Product) -> Result<usize,error> {
                let mut connection = self.connection.get().unwrap();
                diesel::insert_into(product)
                    .values(_dto)
                    .execute(&mut connection)
            }
    
            pub fn update(&self,_dto: &Product,_id: &Uuid) -> Result<usize,error> {
                let mut connection = self.connection.get().unwrap();
                diesel::update(product)
                    .filter(id.eq(_id))
                    .set(_dto)
                    .execute(&mut connection)
            }
            
            pub fn delete(&self,_id: &Uuid) -> Result<usize,error> {
                let mut connection = self.connection.get().unwrap();
                diesel::delete(product)
                    .filter(id.eq(_id))
                    .execute(&mut connection)
            }

            pub fn delete_product_image(&self,_product_id: &Uuid) -> Result<usize,error> {
                let mut connection = self.connection.get().unwrap();
                    diesel::delete(other_product_image)
                        .filter(product_id.eq(product_id))
                        .execute(&mut connection)               
            }
/*
            fn load_products_with_images(&mut self) -> QueryResuilt<Vec<(Product, Vec<ProductImage>)>> {
                let mut connection = self.connection.get().unwrap();
                let items: Vec<Product> = self.get_all()?;
                let all_images: Vec<ProductImage> = ProductImage::belonging_to(&items).load(connection)?;
                let grouped = all_images.grouped_by(&items);
                //// 3) Agrupar por producto correspondiente
                ///let grouped: Vec<Vec<ProductImage>> = all_images.grouped_by(&items);
                Ok(items.into_iter().zip(grouped).collect())
            }
*/

        fn load_products_with_images(&mut self,_id: &Uuid) -> QueryResult<Vec<(Product, Vec<ProductImage>)>> {
            let mut connection = self.connection.get().unwrap();
    
            // Usar join en lugar de belonging_to
            let results = product::table()
                .left_join(product_image::table)
                .filter(id.eq(_id))
                .select((Product::as_select(), Option::<ProductImage>::as_select()))
                .load::<(Product, Option<ProductImage>)>(&mut connection)?;
    
            // Agrupar los resultados
            let mut grouped: std::collections::HashMap<Uuid, (Product, Vec<ProductImage>)> = HashMap::new();
    
            for (prod, img_opt) in results {
                let entry = grouped.entry(prod.id.unwrap()).or_insert((prod, Vec::new()));
                if let Some(img) = img_opt {
                    entry.1.push(img);
                }
            }
    
            Ok(grouped.into_values().collect())
        }

        }
}
