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

#[derive(Deserialize, Debug)]
pub struct S2tAuction {
    success: bool,
    page: i64,
    total_pages: i64,
    total_auctions: i64,
    last_updated: i64,
    auctions: Vec<S2tItem>,
}
