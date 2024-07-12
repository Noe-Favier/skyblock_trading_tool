use crate::auction_response::S2tAuction;
use crate::schema::s2t_item::dsl::*;

lazy_static! {
    static ref LAST_AUCTION: RwLock<Uuid> = RwLock::new(Uuid::nil());
}

use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
    ExpressionMethods, RunQueryDsl,
};
use reqwest::{
    header::{HeaderMap},
    Client,
};
use std::{
    sync::RwLock,
};
use tokio::task;

use lazy_static::lazy_static;
use uuid::Uuid;

pub fn fetch_auction(
    client: Client,
    api_url: String,
    headers: HeaderMap,
    loop_count: u64,
    pool: Pool<ConnectionManager<PgConnection>>,
) {
    task::spawn(async move {
        let res = client
            .get(format!("{}/{}", &api_url, "auctions"))
            .headers(headers)
            .send()
            .await;

        match res {
            Ok(response) => match response.text().await {
                Ok(response_text) => match serde_json::from_str::<S2tAuction>(&response_text) {
                    Ok(auction_response) => {
                        let auction_count = auction_response.auctions.len();
                        println!("{} - ✅ Auction [{}]", loop_count, auction_count);

                        let mut last_auction = LAST_AUCTION.write().unwrap();
                        if last_auction.is_nil() {
                            *last_auction = auction_response.auctions[auction_count - 1].auction_id;
                        }

                        if auction_response.auctions[0].auction_id == *last_auction {
                            println!("{} - ✅ No new auction ({})", loop_count, last_auction);
                        } else {
                            let mut i = 0;
                            let mut new_auctions = 0;
                            while i < auction_count {
                                let item = &auction_response.auctions[i];
                                if item.auction_id == *last_auction {
                                    break;
                                }
                                if item.bin {
                                    println!(
                                        "{} - *️⃣  New auction: {} : {}",
                                        loop_count, item.item_name, item.starting_bid
                                    );
                                    {
                                        let conn = &mut pool.get().unwrap();
                                        if let Err(_err) = diesel::insert_into(s2t_item)
                                            .values((
                                                auction_id.eq(item.auction_id),
                                                item_name.eq(&item.item_name),
                                                item_uuid.eq(item.item_uuid),
                                                category.eq(&item.category),
                                                tier.eq(&item.tier),
                                                item_lore.eq(&item.item_lore),
                                                starting_bid.eq(item.starting_bid),
                                            ))
                                            .execute(conn)
                                        {
                                            //ERROR HANDLING

                                            if _err.to_string().contains("s2t_item_pkey") {
                                                //if primary key violation, we can break the loop
                                                println!(
                                                    "{} - 💫 Primary key violated",
                                                    loop_count
                                                );
                                                break;
                                            } else {
                                                println!(
                                                    "{} - ❌ Error inserting item: {}",
                                                    loop_count, _err
                                                );
                                                i += 1;
                                                continue;
                                            }
                                        }
                                    }

                                    new_auctions += 1;
                                }

                                i += 1;
                            }
                            *last_auction = auction_response.auctions[0].auction_id;
                            println!("{} - ✅ Total new auctions: {}", loop_count, new_auctions);
                        }
                    }
                    Err(e) => eprintln!("❌ Error parsing JSON: {}", e),
                },
                Err(e) => eprintln!("❌ Error reading response text: {}", e),
            },
            Err(e) => eprintln!("❌ Error sending request: {}", e),
        }
    });
}
