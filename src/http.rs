use std::str::FromStr;

use crate::bo::p_s2t_item::PS2tItem;
use crate::dto::page_dto::PageDto;
use crate::schema::p_s2t_item::dsl::p_s2t_item;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use serde::Serialize;
use tokio_cron_scheduler::JobScheduler;
use warp::{http, Filter, Rejection, Reply};

/*
/page/{page_id}
/item/{item_name}
/state
*/

const PAGE_SIZE: i64 = 100;

pub fn start_http_handler(scheduler: JobScheduler, pool: Pool<ConnectionManager<PgConnection>>) {
    let main_route = warp::path::end().map(|| {
        println!("GET request received");
        warp::reply::html("Hello, world!")
    });

    let pool_clone = pool.clone();
    let page_route = warp::path!("page" / i64).and_then(move |page: i64| {
        let pool_clone = pool_clone.clone();
        async move {
            let conn = &mut pool_clone.get().unwrap();

            let items: Vec<PS2tItem> = p_s2t_item
                .limit(PAGE_SIZE)
                .offset(page * PAGE_SIZE)
                .load::<PS2tItem>(conn)
                .expect("Error loading items");

            let items_count: i64 = p_s2t_item
                .count()
                .get_result(conn)
                .expect("Error counting items");

            let p: PageDto = PageDto {
                page: items,
                page_size: PAGE_SIZE,
                total_pages: items_count / PAGE_SIZE,
                total_items: items_count,
            };

            Ok(warp::reply::json(&p)) as Result<_, Rejection>
        }
    });

    let routes = main_route.or(page_route);

    tokio::spawn(async move {
        warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
    });
}
