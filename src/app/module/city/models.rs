pub(crate) mod model {
    use serde::{Deserialize, Serialize};
    use sqlx::FromRow;
    use validator::Validate;

    #[derive(FromRow, Clone, Debug, Default, Validate, Serialize, Deserialize)]
    pub(crate) struct City {
        pub id: i32,
        pub name: String,
        pub ip: String,
    }
}
