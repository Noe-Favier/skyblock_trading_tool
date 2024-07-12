use crate::bo::p_s2t_item::PS2tItem;
use crate::schema::p_s2t_item::dsl::p_s2t_item;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use tokio_cron_scheduler::JobScheduler;
use warp::Filter;

/*
/page/{page_id}
/item/{item_name}
/state
*/

const PAGE_SIZE: i64 = 100;

pub fn start_http_handler(scheduler: JobScheduler, pool: Pool<ConnectionManager<PgConnection>>) {
    let main_route = warp::path::end().map(|| {
        println!("GET request received");
        warp::reply::html("Hello, World!")
    });

    let pool_clone = pool.clone();
    let page_route = warp::path!("page" / i64).map(move |page: i64| {
        let conn = &mut pool_clone.get().unwrap();

        let items: Vec<PS2tItem> = p_s2t_item
            .limit(PAGE_SIZE)
            .offset(page * PAGE_SIZE)
            .load::<PS2tItem>(conn)
            .expect("Error loading items");

        warp::reply::json(&items)
    });

    let routes = main_route.or(page_route);
    //
    tokio::spawn(async move {
        warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
    });
}
