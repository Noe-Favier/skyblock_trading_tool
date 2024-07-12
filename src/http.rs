use crate::dto::page_dto::PageDto;

use tokio_cron_scheduler::JobScheduler;
use warp::Filter;

/*
/page/{page_id}
/item/{item_name}
/state
*/

pub fn start_http_handler(scheduler: JobScheduler) {
    let main_route = warp::path::end().map(|| {
        println!("GET request received");
        warp::reply::html("Hello, World!")
    });

    let page_route = warp::path!("page" / i32).map(|page: i32| {
        println!("GET request received for page: {}", page);
        warp::reply::html(format!("Hello, World! Page: {}", page))
    });

    let routes = main_route.or(page_route);
    //
    tokio::spawn(async move {
        warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
    });
}
