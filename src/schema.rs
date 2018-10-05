table! {
    iot_datas (id) {
        id -> Integer,
        thing_id -> Integer,
        value -> Text,
        created_at -> Timestamp,
    }
}

table! {
    things (id) {
        id -> Integer,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        token -> Text,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    iot_datas,
    things,
);
