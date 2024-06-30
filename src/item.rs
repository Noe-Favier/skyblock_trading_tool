use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct S2tItem {
    item_name: String,
    item_uuid: Uuid,

    category: String,
    tier: String,
    item_lore: String,

    starting_bid: i64,

    bin: bool,
}
