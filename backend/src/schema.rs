diesel::table! {
    cart_items (id) {
        id -> Int4,
        user_id -> Int4,
        uniform_id -> Int4,
        quantity -> Int4,
        added_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    grades (id) {
        id -> Int4,
        #[max_length = 20]
        name -> Varchar,
        school_id -> Int4,
    }
}

diesel::table! {
    order_items (id) {
        id -> Int4,
        order_id -> Int4,
        uniform_id -> Int4,
        quantity -> Int4,
        price -> Float8,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        user_id -> Int4,
        total_price -> Float8,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    payments (id) {
        id -> Int4,
        order_id -> Int4,
        #[max_length = 50]
        payment_method -> Varchar,
        amount -> Float8,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        paid_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    schools (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 255]
        address -> Nullable<Varchar>,
        #[max_length = 20]
        contact_number -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    uniform_categories (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
    }
}

diesel::table! {
    uniforms (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        school_id -> Int4,
        grade_id -> Int4,
        category_id -> Int4,
        #[max_length = 10]
        size -> Nullable<Varchar>,
        price -> Float8,
        stock_quantity -> Nullable<Int4>,
        #[max_length = 255]
        image_url -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 100]
        full_name -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 20]
        role -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(cart_items -> uniforms (uniform_id));
diesel::joinable!(cart_items -> users (user_id));
diesel::joinable!(grades -> schools (school_id));
diesel::joinable!(order_items -> orders (order_id));
diesel::joinable!(order_items -> uniforms (uniform_id));
diesel::joinable!(orders -> users (user_id));
diesel::joinable!(payments -> orders (order_id));
diesel::joinable!(uniforms -> grades (grade_id));
diesel::joinable!(uniforms -> schools (school_id));
diesel::joinable!(uniforms -> uniform_categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    cart_items,
    grades,
    order_items,
    orders,
    payments,
    schools,
    uniform_categories,
    uniforms,
    users,
);