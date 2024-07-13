use crate::bo::p_s2t_item::PS2tItem;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct PageDto {
    pub page_size: i64,
    pub total_pages: i64,
    pub total_items: i64,
    pub items_in_page: i64,
    pub page: Vec<PS2tItem>,
}
