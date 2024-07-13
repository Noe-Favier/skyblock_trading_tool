use crate::bo::p_s2t_item::PS2tItem;
use serde::Serialize;

#[derive(Serialize)]
pub struct PageDto {
    pub page: Vec<PS2tItem>,
    pub page_size: i64,
    pub total_pages: i64,
    pub total_items: i64,
}
