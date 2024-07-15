use std::sync::Arc;

use crate::bo::p_s2t_item::PS2tItem;
use crate::dto::duration_dto::DurationDto;
use crate::dto::item_info_dto::ItemInfoDto;
use crate::dto::page_dto::PageDto;
use crate::dto::state_dto::StateDto;
use crate::schema::p_s2t_item::dsl::p_s2t_item;
use crate::schema::p_s2t_item::dsl::{created_at, item_name, item_name_slug};
use diesel::dsl::count_distinct;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::ExpressionMethods;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use tokio_cron_scheduler::JobScheduler;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::Config;
use warp::{
    http,
    hyper::{Response, StatusCode},
    path::{FullPath, Tail},
    Filter, Rejection, Reply,
};
/*
/page/{page_id}
/item/{item_name}
/state
*/

const PAGE_SIZE: i64 = 100;

#[derive(OpenApi)]
#[openapi(
    paths(page_route, item_route, state_route),
    components(schemas(PS2tItem, ItemInfoDto, PageDto, StateDto, DurationDto))
)]
struct ApiDoc;

pub async fn start_http_handler(
    scheduler: JobScheduler,
    pool: Pool<ConnectionManager<PgConnection>>,
    config: Arc<Config<'static>>,
) {
    let scheduler_filter = warp::any().map(move || scheduler.clone());
    let pool_filter = warp::any().map(move || pool.clone());

    let page_route = warp::path("page")
        .and(warp::path::param())
        .and(pool_filter.clone())
        .and_then(page_route);

    let item_route = warp::path("item")
        .and(warp::path::param())
        .and(pool_filter.clone())
        .and_then(item_route);

    let state_route = warp::path("state")
        .and(scheduler_filter.clone())
        .and_then(state_route);

    let openapi_route = warp::path("api-docs")
        .and(warp::path("openapi.json"))
        .and_then(openapi_route);

    let swaggerui_route = warp::path("swagger-ui")
        .and(warp::get())
        .and(warp::path::full())
        .and(warp::path::tail())
        .and(warp::any().map(move || config.clone()))
        .and_then(serve_swagger);

    let routes = swaggerui_route
        .or(page_route)
        .or(item_route)
        .or(state_route)
        .or(openapi_route);

    let cors = warp::cors().allow_any_origin();
    warp::serve(routes.with(cors))
        .run(([0, 0, 0, 0], 3030))
        .await;
}

#[utoipa::path(
    get,
    path = "/page/{page}",
    responses(
        (status = 200, description = "Page of items", body = PageDto)
    ),
    params(
        ("page" = i64, Path, description = "Page number")
    )
)]
pub async fn page_route(
    page: i64,
    pool: Pool<ConnectionManager<PgConnection>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = &mut pool.get().unwrap();

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

#[utoipa::path(
    get,
    path = "/item/{item_name}",
    responses(
        (status = 200, description = "Item info", body = ItemInfoDto)
    ),
    params(
        ("item_name" = String, Path, description = "Item name")
    )
)]
pub async fn item_route(
    rq_item_name: String,
    pool_clone: Pool<ConnectionManager<PgConnection>>,
) -> Result<impl warp::Reply, warp::Rejection> {
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

#[utoipa::path(
    get,
    path = "/state",
    responses(
        (status = 200, description = "Scheduler state", body = StateDto)
    )
)]
pub async fn state_route(mut scheduler: JobScheduler) -> Result<impl warp::Reply, warp::Rejection> {
    let t = scheduler
        .time_till_next_job()
        .await
        .expect("Error getting time till next job");

    let state = StateDto {
        time_before_compil: DurationDto {
            secs: t.unwrap().as_secs(),
        },
    };

    Ok(warp::reply::json(&state)) as Result<_, Rejection>
}

#[utoipa::path(
    get,
    path = "/api-docs/openapi.json",
    responses(
        (status = 200, description = "OpenAPI schema", body = String)
    )
)]
pub async fn openapi_route() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_header(
        ApiDoc::openapi().to_pretty_json().unwrap(),
        http::header::CONTENT_TYPE,
        "application/json",
    )) as Result<_, Rejection>
}

async fn serve_swagger(
    full_path: FullPath,
    tail: Tail,
    config: Arc<Config<'static>>,
) -> Result<Box<dyn Reply + 'static>, Rejection> {
    if full_path.as_str() == "/swagger-ui" {
        return Ok(Box::new(warp::redirect::found(http::Uri::from_static(
            "/swagger-ui/",
        ))));
    }

    let path = tail.as_str();
    match utoipa_swagger_ui::serve(path, config.clone()) {
        Ok(file) => {
            if let Some(file) = file {
                Ok(Box::new(
                    Response::builder()
                        .header("Content-Type", file.content_type)
                        .body(file.bytes),
                ))
            } else {
                Ok(Box::new(StatusCode::NOT_FOUND))
            }
        }
        Err(error) => Ok(Box::new(
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(error.to_string()),
        )),
    }
}
