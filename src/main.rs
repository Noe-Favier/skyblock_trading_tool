mod auction_response;
mod item;

use auction_response::S2tAuction;
use dotenv::dotenv;
use lazy_static::lazy_static;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use std::{env::var, sync::RwLock};
use tokio::task;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

lazy_static! {
    static ref LAST_AUCTION: RwLock<Uuid> = RwLock::new(Uuid::nil());
}

#[tokio::main]
async fn main() {
    println!("--- starting ---");

    dotenv().ok();
    let api_key: String = var("API_KEY").expect("API_KEY not set");
    let api_url: String = var("API_URL").expect("API_URL not set");
    println!("✅ API_KEY: {}", api_key);
    println!("✅ API_URL: {}", api_url);

    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "API-Key",
        HeaderValue::from_str(&api_key).expect("Invalid API key"),
    );
    println!("✅ headers: {:?}", headers);

    println!("--- xxxxxxxx ---");

    let mut loop_count: u64 = 0;
    loop {
        loop_count += 1;
        println!("--- loop {} ---", loop_count);

        let client_clone = client.clone();
        let headers_clone = headers.clone();
        let api_url_clone = api_url.clone();

        task::spawn(async move {
            let res = client_clone
                .get(format!("{}/{}", &api_url_clone, "auctions"))
                .headers(headers_clone)
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
                                *last_auction =
                                    auction_response.auctions[auction_count - 1].auction_id;
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
                                            "{} - ✅ New auction: {} : {}",
                                            loop_count, item.item_name, item.starting_bid
                                        );
                                        new_auctions += 1;
                                    }
                                    i += 1;
                                }
                                *last_auction = auction_response.auctions[0].auction_id;
                                println!(
                                    "{} - ✅ Total new auctions: {}",
                                    loop_count, new_auctions
                                );
                            }
                        }
                        Err(e) => eprintln!("❌ Error parsing JSON: {}", e),
                    },
                    Err(e) => eprintln!("❌ Error reading response text: {}", e),
                },
                Err(e) => eprintln!("❌ Error sending request: {}", e),
            }
        });

        sleep(Duration::from_secs(1)).await;
    }
}
