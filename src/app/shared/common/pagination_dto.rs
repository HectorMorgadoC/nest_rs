pub mod pagination_dto {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Pagination {
        pub limit: i32,
        pub offset: i32,
    }
}
