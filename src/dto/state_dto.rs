use std::time::Duration;

use serde::Serialize;
use utoipa::ToSchema;

use super::duration_dto::DurationDto;

#[derive(Serialize, ToSchema)]
pub struct StateDto {
    pub time_before_compil: DurationDto,
}
