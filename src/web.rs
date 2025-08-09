use crate::tx;
use axum::{
    Router,
    extract::State,
    response::Json,
    routing::{get, post},
};
use bitcoin::secp256k1::{Keypair, PublicKey};
use bitcoin::{Transaction, consensus::deserialize};
use serde::Deserialize;
use serde_json::json;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;

use bitcoin::Amount;

use crate::cashu;

#[derive(Clone)]
pub struct AppState {
    pub mint_url: String,
}

lazy_static::lazy_static! {
    static ref SERVER_PK: PublicKey = PublicKey::from_str("030c99c81e19622d30cc5fe697f65f03eed93bbf177df99065a2d1f3141dcd095e").unwrap();
    static ref USER_KEY: Keypair = Keypair::from_str("b90c5fa5e7c920f2582d67a440a3f2b1c09fff588f42b0a04754f1e7fb63a2d2")
            .unwrap();
}

lazy_static::lazy_static! {
    static ref TX: Mutex<Option<Transaction>> = Mutex::new(None);
    static ref CHANNEL_AMT: Mutex<Amount> = Mutex::new(Amount::ZERO);
    static ref CHANNEL_BALANCE: Mutex<Amount> = Mutex::new(Amount::ZERO);
}

pub fn create_app(mint_url: String) -> Router {
    let state = AppState { mint_url };

    Router::new()
        .route("/address", get(get_address))
        .route("/tx", post(submit_tx))
        .route("/token", post(get_token))
        .layer(CorsLayer::permissive())
        .with_state(Arc::new(state))
}

async fn get_address() -> Json<serde_json::Value> {
    Json(json!({
        "address": tx::create_address(*SERVER_PK, USER_KEY.public_key()).to_string(),
    }))
}

#[derive(Deserialize)]
struct TxRequest {
    tx: String,
}

async fn submit_tx(Json(payload): Json<TxRequest>) -> Json<serde_json::Value> {
    let parsed: Transaction = deserialize(&hex::decode(payload.tx).unwrap()).unwrap();
    let mut tx = TX.lock().unwrap();
    *tx = Some(parsed.clone());

    let spk = tx::create_spk(*SERVER_PK, USER_KEY.public_key());
    let output_idx = parsed
        .output
        .iter()
        .position(|o| o.script_pubkey == spk)
        .unwrap();

    *CHANNEL_AMT.lock().unwrap() = parsed.output[output_idx].value;
    *CHANNEL_BALANCE.lock().unwrap() = parsed.output[output_idx].value;

    Json(json!({
        "status": "success",
    }))
}

#[derive(Deserialize)]
struct TokenRequest {
    amount: u64,
}

async fn get_token(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<TokenRequest>,
) -> Json<serde_json::Value> {
    // Use the mint URL from the application state
    let token = cashu::get_token(payload.amount, &state.mint_url)
        .await
        .unwrap();

    let mut bal = CHANNEL_BALANCE.lock().unwrap();
    *bal -= Amount::from_sat(payload.amount);

    let update_tx = tx::update(
        *USER_KEY,
        *CHANNEL_AMT.lock().unwrap() - *bal,
        *bal,
        *SERVER_PK,
        TX.lock().unwrap().as_ref().unwrap(),
    );

    let tx_json = tx::tx_json(&update_tx);

    Json(json!({
        "token": token.to_string(),
        "update_tx": tx_json,
    }))
}
