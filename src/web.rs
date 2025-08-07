use crate::tx;
use axum::{
    Router,
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::{get, post},
};
use bitcoin::secp256k1::{Keypair, PublicKey};
use bitcoin::{Transaction, consensus::deserialize};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;

use crate::cashu;

lazy_static::lazy_static! {
    static ref SERVER_PK: PublicKey = PublicKey::from_str("030c99c81e19622d30cc5fe697f65f03eed93bbf177df99065a2d1f3141dcd095e").unwrap();
}

lazy_static::lazy_static! {
    static ref TX: Mutex<Option<Transaction>> = Mutex::new(None);
}

pub fn create_app() -> Router {
    Router::new()
        .route("/address", get(get_address))
        .route("/tx", post(submit_tx))
        .route("/token", post(get_token))
        .layer(CorsLayer::permissive())
}

async fn get_address() -> Json<serde_json::Value> {
    let user_key =
        Keypair::from_str("b90c5fa5e7c920f2582d67a440a3f2b1c09fff588f42b0a04754f1e7fb63a2d2")
            .unwrap();

    Json(json!({
        "address": tx::create_address(*SERVER_PK, user_key.public_key()).to_string(),
    }))
}

#[derive(Deserialize)]
struct TxRequest {
    tx: String,
}

async fn submit_tx(Json(payload): Json<TxRequest>) -> Json<serde_json::Value> {
    let parsed: Transaction = deserialize(&hex::decode(payload.tx).unwrap()).unwrap();
    let mut tx = TX.lock().unwrap();
    *tx = Some(parsed);

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
