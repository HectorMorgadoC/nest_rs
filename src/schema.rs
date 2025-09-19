// @generated automatically by Diesel CLI.

diesel::table! {
    product (id) {
        id -> Nullable<Uuid>,
        #[max_length = 50]
        title -> Varchar,
        price -> Float4,
        #[max_length = 200]
        description -> Varchar,
        #[max_length = 50]
        slug -> Varchar,
        stock -> Int4,
        #[max_length = 50]
        gender -> Varchar,
    }
}

diesel::table! {
    product_image (id) {
        id -> Nullable<Uuid>,
        #[max_length = 200]
        url -> Varchar,
        product_id -> Uuid,
    }
}

diesel::table! {
    user_auth (id) {
        id -> Nullable<Uuid>,
        #[max_length = 30]
        email -> Varchar,
        #[max_length = 60]
        password -> Varchar,
        #[max_length = 50]
        fullname -> Varchar,
        is_active -> Bool,
        roles -> Array<Varchar>,
    }
}

diesel::joinable!(product_image -> product (product_id));

diesel::allow_tables_to_appear_in_same_query!(product, product_image, user_auth,);
