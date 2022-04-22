table! {
    prices (id) {
        id -> Int4,
        name -> Varchar,
        user_id -> Int4,
    }
}

table! {
    prices_products (id) {
        id -> Int4,
        price_id -> Int4,
        product_id -> Int4,
        user_id -> Int4,
        amount -> Nullable<Int4>,
    }
}

table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        stock -> Float8,
        cost -> Nullable<Int4>,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        company -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(prices -> users (user_id));
joinable!(prices_products -> prices (price_id));
joinable!(prices_products -> products (product_id));
joinable!(prices_products -> users (user_id));
joinable!(products -> users (user_id));

allow_tables_to_appear_in_same_query!(
    prices,
    prices_products,
    products,
    users,
);
