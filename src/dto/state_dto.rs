use std::time::Duration;

use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct StateDto {
    pub time_before_compil: Duration,
}
