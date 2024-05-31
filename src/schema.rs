// @generated automatically by Diesel CLI.

diesel::table! {
    alerts (id) {
        id -> Int4,
        symbol -> Varchar,
        basis -> Varchar,
        ma_length -> Nullable<Int4>,
        value -> Float8,
        direction -> Varchar,
        status -> Varchar,
    }
}
