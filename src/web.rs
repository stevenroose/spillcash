use axum::{
    Router,
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use crate::cashu;

#[derive(Clone)]
struct AppState {}

pub fn create_app() -> Router {
    Router::new()
        .route("/address", get(get_address))
        .route("/tx", post(submit_tx))
        .route("/token", post(get_token))
        .with_state(Arc::new(AppState {}))
        .layer(CorsLayer::permissive())
}

async fn get_address() -> Json<serde_json::Value> {
    Json(json!({
        "address": "bc1",
    }))
}

#[derive(Deserialize)]
struct TxRequest {
    tx: String,
}

async fn submit_tx(Json(payload): Json<TxRequest>) -> Json<serde_json::Value> {
    Json(json!({
        "status": "success",
    }))
}

#[derive(Deserialize)]
struct TokenRequest {
    amount: u64,
}

async fn get_token(Json(payload): Json<TokenRequest>) -> Json<serde_json::Value> {
    let token = cashu::get_token(payload.amount).await.unwrap();
    Json(json!({
        "token": token.to_string(),
    }))
}
