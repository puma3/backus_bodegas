table! {
    stores (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
        lat -> Float8,
        lng -> Float8,
        address -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        country_code -> Nullable<Varchar>,
        icon -> Nullable<Varchar>,
        code -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        delivery -> Nullable<Int4>,
        store_url -> Nullable<Varchar>,
        promo1 -> Nullable<Varchar>,
        promo2 -> Nullable<Varchar>,
        promo3 -> Nullable<Varchar>,
        data -> Nullable<Varchar>,
    }
}
