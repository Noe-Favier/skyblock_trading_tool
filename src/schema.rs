// @generated automatically by Diesel CLI.

diesel::table! {
    p_s2t_item (id) {
        id -> Int4,
        created_at -> Timestamp,
        item_name -> Text,
        item_uuid -> Nullable<Uuid>,
        category -> Text,
        tier -> Text,
        bid -> Int8,
        sell_number -> Int8,
    }
}

diesel::table! {
    s2t_item (auction_id) {
        auction_id -> Uuid,
        item_name -> Text,
        item_uuid -> Nullable<Uuid>,
        category -> Text,
        tier -> Text,
        item_lore -> Text,
        starting_bid -> Int8,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    p_s2t_item,
    s2t_item,
);
