use crate::schema::p_s2t_item::dsl::p_s2t_item;
use crate::schema::p_s2t_item::dsl::*;
use chrono::NaiveDateTime;
use diesel::{Queryable, Selectable};
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = p_s2t_item)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PS2tItem {
    #[diesel(sql_type = diesel::sql_types::BigInt, column_name = "id")]
    pub id: i32,
    #[diesel(sql_type = diesel::sql_types::Timestamp)]
    pub created_at: NaiveDateTime,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub item_name: String,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Uuid>)]
    pub item_uuid: Option<Uuid>,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub category: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub tier: String,
    #[diesel(sql_type = diesel::sql_types::Int8)]
    pub bid: i64,
    #[diesel(sql_type = diesel::sql_types::Int8)]
    pub sell_number: i64,
}
