use axum::{
    extract::{Json, Query, State},
    http::{HeaderValue, Method},
    routing::{get, post},
    Router,
};
use sqlx::MySqlPool;
use tower_http::cors::CorsLayer;

mod error;
mod pixiu;
use error::AppError;

pub fn app(pool: MySqlPool) -> Router {
    Router::new()
        .route("/pixiu/init", post(pixiu_init))
        .route("/pixiu/fund", post(pixiu_insert_fund_info))
        .route("/pixiu/fund", get(pixiu_get_fund_info))
        .route("/pixiu/debt", get(pixiu_get_debt_info))
        .route("/pixiu/property", get(pixiu_get_property_info))
        .with_state(pool.clone())
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::OPTIONS,
                    Method::HEAD,
                    Method::PUT,
                ]),
        )
}

async fn pixiu_init(State(pool): State<MySqlPool>) -> Result<(), AppError> {
    pixiu::init(&pool).await?;
    Ok(())
}

async fn pixiu_insert_fund_info(
    State(pool): State<MySqlPool>,
    Json(payload): Json<pixiu::FundInfo>,
) -> Result<(), AppError> {
    pixiu::insert_fund_info(&pool, payload).await?;
    Ok(())
}

async fn pixiu_get_fund_info(
    State(pool): State<MySqlPool>,
    Query(params): Query<PageRequest>,
) -> Result<Json<PageResponse<pixiu::FundInfo>>, AppError> {
    let total = pixiu::count(&pool, params.from, params.to).await?;
    let funds =
        pixiu::get_fund_info(&pool, params.from, params.to, params.page, params.size).await?;
    let sums = pixiu::get_sum_info(&pool, params.from, params.to).await?;
    let income = pixiu::get_income_info(&pool, params.from, params.to).await?;
    let expenses = pixiu::get_expense_info(&pool, params.from, params.to).await?;
    let response = PageResponse {
        total,
        data: funds,
        sum: sums,
        income,
        expenses,
    };
    Ok(Json(response))
}

async fn pixiu_get_debt_info(
    State(pool): State<MySqlPool>,
) -> Result<Json<Vec<pixiu::DebtInfo>>, AppError> {
    let debts = pixiu::get_debt_info(&pool).await?;
    Ok(Json(debts))
}

async fn pixiu_get_property_info(
    State(pool): State<MySqlPool>,
) -> Result<Json<Vec<pixiu::PropertyInfo>>, AppError> {
    let properties = pixiu::get_property_info(&pool).await?;
    Ok(Json(properties))
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PageRequest {
    from: i64,
    to: i64,
    page: u32,
    size: u32,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PageResponse<T> {
    total: i32,
    data: Vec<T>,
    sum: Vec<pixiu::SumInfo>,
    income: f32,
    expenses: f32,
}
