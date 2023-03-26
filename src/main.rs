use axum::{
    extract::State,
    headers::ContentType,
    http::{HeaderName, HeaderValue, Response, StatusCode},
    response::IntoResponse,
    routing::get,
    Router, TypedHeader,
};
use bigdecimal::BigDecimal;
use dotenv::dotenv;
use serde::{Deserialize, Serialize, __private::de::Content};
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};
use std::env;
use std::net::SocketAddr;

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Coach {
    id: i32,
    solo_rate_cents_per_minute: i32,
    group_rate_cents_per_minute: i32,
    tax_rate_percent: BigDecimal,
    company_name: String,
    company_street: String,
    company_town: String,
    company_province: String,
    company_country: String,
    company_postal_code: String,
    sales_tax_number: Option<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect(&format!(
            "Unable to connect to database at url:{}",
            database_url
        ));

    let app = Router::new()
        .route("/", get(root))
        .route("/coaches", get(get_coaches))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    async fn root() -> &'static str {
        "Baddass Billing 2.0"
    }
    async fn get_coaches(
        State(pool): State<PgPool>,
    ) -> Result<impl IntoResponse, (StatusCode, String)> {
        let coaches = sqlx::query_as::<_, Coach>(
            r#"
        SELECT
            id,
            solo_rate_cents_per_minute,
            group_rate_cents_per_minute,
            tax_rate_percent,
            company_name,
            company_street,
            company_town,
            company_province,
            company_country,
            company_postal_code,
            sales_tax_number
        FROM coaches
        "#,
        )
        .fetch_all(&pool)
        .await
        .map_err(internal_error)?;

        let body = serde_json::to_string(&coaches).map_err(|e| {
            tracing::error!("Error serializing coaches to JSON: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error serializing coaches to JSON".to_string(),
            )
        })?;

        let res = (StatusCode::OK, TypedHeader(ContentType::json()), body).into_response();
        Ok(res)
    }
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
