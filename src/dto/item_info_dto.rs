use serde::Serialize;

use crate::bo::p_s2t_item::PS2tItem;

#[derive(Serialize)]
pub struct ItemInfoDto {
    item: PS2tItem,
    image_url: String,
}
