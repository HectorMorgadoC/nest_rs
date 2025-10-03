pub(crate) mod model {
    pub mod dto {
        use mongodb::bson::oid::ObjectId;
        use serde::{Deserialize, Serialize};
        use validator::Validate;

        #[derive(Serialize, Debug, Deserialize, Clone, Validate)]
        pub struct Team {
            #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
            pub id: Option<ObjectId>,
            pub name: String,
            pub price: u8,
        }
    }
}
