// @generated automatically by Diesel CLI.

diesel::table! {
    articles (hash) {
        hash -> Nullable<Text>,
        title -> Nullable<Text>,
        version -> Integer,
        path -> Text,
        feed -> Integer,
        url -> Text,
    }
}
