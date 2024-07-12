#[macro_use]
extern crate diesel_migrations;

use std::{
    env::var
    ,
    sync::{Arc, Mutex},
};

use diesel::{
    ExpressionMethods,
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool}, RunQueryDsl,
};
use diesel_migrations::{embed_migrations, MigrationHarness};
use diesel_migrations::EmbeddedMigrations;
use dotenv::dotenv;
use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue},
};
use tokio::time::{Duration, sleep};
use tokio_cron_scheduler::{Job, JobScheduler};
use warp::Filter;

mod auction_response;
mod fetch;
mod http;
mod item;
mod schema;
mod dto;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
pub fn get_connection_pool(url: String) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(url);

    return Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("❌ Could not build connection pool");
}

#[tokio::main]
async fn main() {
    println!("--- starting ---");

    dotenv().ok();
    let api_key: String = var("API_KEY").expect("API_KEY not set");
    let api_url: String = var("API_URL").expect("API_URL not set");
    let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("✅ API_KEY: {}", api_key);
    println!("✅ API_URL: {}", api_url);
    println!("✅ DATABASE_URL: {}", database_url);

    // MIGRATION
    let pool = get_connection_pool(database_url);
    {
        let conn = &mut pool.get().unwrap();
        conn.run_pending_migrations(MIGRATIONS)
            .expect("❌ Error running migrations");
    }

    // CRON JOB
    let sched = {
        let result = JobScheduler::new().await;
        match result {
            Ok(scheduler) => scheduler,
            Err(err) => {
                eprintln!("❌ Error creating job scheduler: {}", err);
                return;
            }
        }
    };
    let conn = pool.get().unwrap();
    let conn_mutex = Arc::new(Mutex::new(conn));
    let job = Job::new("0 * */6 * * * *", move |_uuid, _lock| {
        // Every 6 hours
        println!("🔔 Compile job triggered ...");

        let mut conn_lock = conn_mutex.lock().unwrap();
        conn_lock
            .build_transaction()
            .read_write()
            .run::<_, diesel::result::Error, _>(|conn_lock| {
                diesel::sql_query("CALL compile_items()").execute(&mut *conn_lock)?;
                println!("🔔 ❇️  Items compiled");
                Ok(())
            })
            .expect("🔔 ❌  Error compiling items");
    })
    .expect("❌ Error creating job");

    match sched.add(job).await {
        Ok(_) => println!("✅ 🔔 JOB COMPILE ADDED"),
        Err(err) => eprintln!("❌ Error adding job: {}", err),
    }
    sched.start().await.expect("❌ Error starting scheduler");

    // HTTP CLIENT

    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "API-Key",
        HeaderValue::from_str(&api_key).expect("Invalid API key"),
    );
    println!("✅ headers: {:?}", headers);

    // HTTP HANDLER
    http::start_http_handler(sched);

    // INDEXER

    println!("--- xxxxxxxx ---");
    let mut loop_count: u64 = 0;
    loop {
        loop_count += 1;
        println!("--- loop {} ---", loop_count);
        fetch::fetch_auction(
            client.clone(),
            api_url.clone(),
            headers.clone(),
            loop_count.clone(),
            pool.clone(),
        );
        sleep(Duration::from_secs(1)).await;
    }
}
