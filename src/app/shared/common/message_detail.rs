
pub mod message_detail {
    
    use serde::Serialize;
    use std::fmt;
    
    #[derive(Serialize,Debug)]
        pub struct Details {
            pub title: String,
            pub status: u16,
            pub detail: String,
            pub instance: String
        }

    impl fmt::Display for Details {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{} ({}): {} [{}]",
                self.title,
                self.status,
                self.detail,
                self.instance
            )?; 

            Ok(())
        }
    }

   
    impl Details {
        pub fn new(title: String, status: u16, detail: String, instance: String) -> Self {
            Details {
                title,
                status,
                detail,
                instance
            }
        }
        
    }
 
}
