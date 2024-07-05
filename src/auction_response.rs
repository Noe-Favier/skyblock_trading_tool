use crate::item::S2tItem;
use serde::Deserialize;
/*
Sample :

{
"success": true,
"page": 0,
"totalPages": 32,
"totalAuctions": 31267,
"lastUpdated": 1571065561345,
"auctions": [
    {S2tItem}, ...
]
}

*/

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct S2tAuction {
    pub success: bool,
    pub page: i64,
    pub total_pages: i64,
    pub total_auctions: i64,
    pub last_updated: i64,
    pub auctions: Vec<S2tItem>,
}
