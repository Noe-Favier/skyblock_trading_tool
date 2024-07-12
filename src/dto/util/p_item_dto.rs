use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct PItem{
    id : u64,
    created_at :String,
    item_name :String,
    item_uuid :Option<Uuid>,
    category :String,
    tier :String,
    bid :u64,
    sell_number : u64,
}