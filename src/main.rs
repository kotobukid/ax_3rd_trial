use std::str::FromStr;
use tokio;
use std::net::{SocketAddr};
use axum::{
    response::IntoResponse,
    routing::{
        get,
        post,
    },
    Router,
    Server,
    http::StatusCode,
    extract::{
        Query,
        Json,
    },
};
use serde::{
    Deserialize
};
use axum_extra::extract::cookie::CookieJar;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();

    let app = Router::new()
        .route("/", get(home_handler))
        .route("/q", get(query_parse_handler).post(body_parse_handler))
        .route("/c", get(cookie_parse_handler))
        .route("/cq", get(cookie_and_query))
        .route("/both", post(q_and_body))
        ;

    println!("{}", &addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn home_handler() -> impl IntoResponse {
    (StatusCode::OK, "Index page")
}

#[derive(Debug, Deserialize, Clone, Copy, Default)]
struct QuerySample1 {
    id: i32,
}

async fn query_parse_handler(query: Query<QuerySample1>) -> impl IntoResponse {
    println!("{:?}", query);
    (StatusCode::OK, format!("id is {}", query.id))
}

async fn body_parse_handler(body: Json<QuerySample1>) -> impl IntoResponse {
    println!("{:?}", body);
    (StatusCode::OK, format!("id is {}", body.id))
}

// async fn q_and_body(body: Json<QuerySample1>, query: Query<QuerySample1>) -> impl IntoResponse { // compile error
async fn q_and_body(query: Query<QuerySample1>, body: Json<QuerySample1>) -> impl IntoResponse {
    println!("{:?}", body);
    println!("{:?}", query);
    (StatusCode::OK, format!("id is {}, {}", body.id, query.id))
}

async fn cookie_parse_handler(cookie: CookieJar) -> impl IntoResponse {
    println!("{:?}", cookie);
    if let Some(sid) = cookie.get("sid") {
        (StatusCode::OK, format!("id is {}", sid))
    } else {
        (StatusCode::UNAUTHORIZED, "not authorized".into())
    }
}

async fn cookie_and_query(cookie: CookieJar, query: Query<QuerySample1>) -> impl IntoResponse {
    println!("{:?}", cookie);
    println!("{:?}", query);
    let qid: i32 = query.id;
    if let Some(sid) = cookie.get("sid") {
        (StatusCode::OK, format!("sid(cookie) is {}, id(query) is {}", sid, qid))
    } else {
        (StatusCode::UNAUTHORIZED, "not authorized".into())
    }
}