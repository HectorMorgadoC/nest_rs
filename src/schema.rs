// @generated automatically by Diesel CLI.

diesel::table! {
    product (id) {
        id -> Uuid,
        #[max_length = 50]
        title -> Varchar,
        price -> Float8,
        #[max_length = 200]
        description -> Varchar,
        #[max_length = 50]
        slug -> Nullable<Varchar>,
        stock -> Int4,
        sizes -> Nullable<Array<Nullable<Text>>>,
        #[max_length = 50]
        gender -> Nullable<Varchar>,
    }
}

diesel::table! {
    product_image (id) {
        id -> Uuid,
        #[max_length = 200]
        url -> Varchar,
        product_id -> Uuid,
    }
}

diesel::joinable!(product_image -> product (product_id));

diesel::allow_tables_to_appear_in_same_query!(
    product,
    product_image,
);
