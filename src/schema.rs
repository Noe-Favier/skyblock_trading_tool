// @generated automatically by Diesel CLI.

diesel::table! {
    s2t_item (auction_id) {
        auction_id -> Uuid,
        item_name -> Text,
        item_uuid -> Nullable<Uuid>,
        category -> Text,
        tier -> Text,
        item_lore -> Text,
        starting_bid -> Int8,
    }
}
