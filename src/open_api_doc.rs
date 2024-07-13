use crate::dto::{item_info_dto::ItemInfoDto, page_dto::PageDto, state_dto::StateDto};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(components(schemas(PageDto, ItemInfoDto, StateDto)))]
pub struct ApiDoc;
