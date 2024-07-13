use serde::Serialize;
use utoipa::ToSchema;

use crate::bo::p_s2t_item::PS2tItem;

#[derive(Serialize, ToSchema)]
pub struct ItemInfoDto {
    pub item: PS2tItem,
    pub versions: Vec<PS2tItem>,
    pub image_url: String,
}
