use serde::Serialize;
use crate::dto::util::p_item_dto::PItem;
use crate::schema::p_s2t_item::dsl::p_s2t_item;

#[derive(Serialize)]
pub struct PageDto {
    pub page: Vec<PItem>,
    pub page_size: u64,
    pub total_pages: u64,
    pub total_items: u64,
}
