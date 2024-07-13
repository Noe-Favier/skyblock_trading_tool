use crate::bo::p_s2t_item::PS2tItem;
use crate::dto::item_info_dto::ItemInfoDto;
use crate::dto::page_dto::PageDto;
use crate::dto::state_dto::StateDto;
use crate::open_api_doc::ApiDoc;
use crate::schema::p_s2t_item::dsl::p_s2t_item;
use crate::schema::p_s2t_item::dsl::{created_at, item_name, item_name_slug};
use diesel::dsl::count_distinct;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::ExpressionMethods;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use tokio_cron_scheduler::JobScheduler;
use utoipa::OpenApi;
use warp::{http, Filter, Rejection};

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
                .distinct_on(item_name)
                .limit(PAGE_SIZE)
                .order_by((item_name, created_at.desc()))
                .offset(page * PAGE_SIZE)
                .load::<PS2tItem>(conn)
                .expect("Error loading items");

            let items_count: i64 = p_s2t_item
                .select(count_distinct(item_name))
                .get_result(conn)
                .expect("Error counting items");

            let p: PageDto = PageDto {
                page_size: PAGE_SIZE,
                total_pages: items_count / PAGE_SIZE,
                total_items: items_count,
                items_in_page: items.len() as i64,
                page: items,
            };

            Ok(warp::reply::json(&p)) as Result<_, Rejection>
        }
    });

    let pool_clone = pool.clone();
    let item_route = warp::path!("item" / String).and_then(move |rq_item_name: String| {
        let pool_clone = pool_clone.clone();
        async move {
            let conn = &mut pool_clone.get().unwrap();

            let item: PS2tItem = p_s2t_item
                .filter(item_name_slug.eq(&rq_item_name))
                .order_by(created_at.desc())
                .first::<PS2tItem>(conn)
                .expect("Error loading item");

            let versions: Vec<PS2tItem> = p_s2t_item
                .filter(item_name.eq(&item.item_name))
                .order_by(created_at.desc())
                .offset(1)
                .load::<PS2tItem>(conn)
                .expect("Error loading versions");

            let info = ItemInfoDto {
                item: item,
                versions: versions,
                image_url: "".to_string(),
            };

            Ok(warp::reply::json(&info)) as Result<_, Rejection>
        }
    });

    let state_route = warp::path("state").and_then(move || {
        let mut scheduler_clone = scheduler.clone();
        async move {
            let t = scheduler_clone
                .time_till_next_job()
                .await
                .expect("Error getting time till next job");

            let state = StateDto {
                time_before_compil: t.unwrap(),
            };

            Ok(warp::reply::json(&state)) as Result<_, Rejection>
        }
    });

    let openapi_route = warp::path("openapi.json").and_then(|| async move {
        Ok(warp::reply::with_header(
            ApiDoc::openapi().to_pretty_json().unwrap(),
            http::header::CONTENT_TYPE,
            "application/json",
        )) as Result<_, Rejection>
    });

    let routes = main_route
        .or(page_route)
        .or(item_route)
        .or(state_route)
        .or(openapi_route);

    tokio::spawn(async move {
        warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
    });
}
