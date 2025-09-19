pub(crate) mod model {
    pub mod dto {
        use diesel::{
            Selectable,
            prelude::{Insertable, Queryable},
        };
        use serde::{Deserialize, Serialize};
        use uuid::Uuid;
        use validator::Validate;

        use crate::schema::user_auth;

        #[derive(
            Queryable,
            Selectable,
            Debug,
            Clone,
            Serialize,
            Deserialize,
            Validate,
            Insertable,
            Default,
        )]
        #[diesel(table_name = user_auth)]
        #[diesel(check_for_backend(diesel::pg::Pg))]
        pub struct User {
            pub id: Option<Uuid>,
            #[validate(length(min = 6, max = 30))]
            pub email: String,
            #[validate(length(min = 8, max = 60))]
            pub password: String,
            #[validate(length(min = 8, max = 50))]
            pub fullname: String,
            pub is_active: bool,
            pub roles: Vec<String>,
        }
    }

    pub mod authorization {
        use serde::{Deserialize, Serialize};
        use validator::Validate;

        #[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
        pub struct User {
            #[validate(length(min = 6, max = 30))]
            pub email: String,
            #[validate(length(min = 8, max = 60))]
            pub password: String,
        }
    }
}
