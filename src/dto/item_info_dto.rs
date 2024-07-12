use serde::Serialize;

use crate::dto::util::p_item_dto::PItem;

#[derive(Serialize)]
pub struct ItemInfoDto {
    item: PItem,
    image_url: String,
}