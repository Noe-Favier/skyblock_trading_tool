mod auction_response;
mod item;

use crate::item::S2tItem;
use auction_response::S2tAuction;

use dotenv::dotenv;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::Deserialize;
use std::env::var;
use tokio::task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("--- starting ---");
    //env
    dotenv().ok();
    let api_key: String = var("API_KEY").unwrap();
    let api_url: String = var("API_URL").unwrap();
    println!("✅ API_KEY: {}", api_key);
    println!("✅ API_URL: {}", api_url);

    //reqwest
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("API-Key", HeaderValue::from_str(&api_key).unwrap());
    println!("✅ headers: {:?}", headers);

    println!("--- xxxxxxxx ---");

    loop {
        println!("--- loop ---");
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
                Ok(_) => {
                    let response_text = res.unwrap().text().await.unwrap();
                    println!("✅ response_text: {:?}", response_text);
                    let i: S2tAuction = serde_json::from_str(&response_text).unwrap();
                    println!("✅ {:?}", i);
                }
                Err(e) => {
                    eprintln!("❌ Error: {}", e);
                }
            }
        });

        sleep(Duration::from_secs(1)).await;
    }
}
