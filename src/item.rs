use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct S2tItem {
    pub item_name: String,
    pub item_uuid: Option<Uuid>,
    pub category: String,
    pub tier: String,
    pub item_lore: String,
    pub starting_bid: i64,
    pub bin: bool,
}
