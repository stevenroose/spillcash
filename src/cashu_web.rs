use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use maud::{html, Markup, PreEscaped, DOCTYPE};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use crate::cashu;

#[derive(Clone)]
pub struct AppState {
    pub mint_url: String,
}

#[derive(Deserialize)]
struct MintForm {
    amount: Option<u64>,
}

#[derive(Deserialize)]
struct TokenForm {
    amount: u64,
}

#[derive(Deserialize)]
struct CompleteMintForm {
    quote_id: String,
}

pub fn create_cashu_app(mint_url: String) -> Router {
    let state = AppState { mint_url };
    
    Router::new()
        .route("/", get(index))
        .route("/mint", post(handle_mint))
        .route("/mint-quote", post(create_mint_quote_handler))
        .route("/complete-mint", post(complete_mint_handler))
        .route("/balance", get(get_balance))
        .route("/token", post(create_token))
        .layer(CorsLayer::permissive())
        .with_state(Arc::new(state))
}

fn base_template(title: &str, content: Markup, balance: u64) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) }
                style {
                    (PreEscaped(r#"
                        :root {
                            font-family: system-ui, Avenir, Helvetica, Arial, sans-serif;
                            line-height: 1.5;
                            font-weight: 400;
                            color-scheme: light dark;
                            color: rgba(255, 255, 255, 0.87);
                            background-color: #242424;
                            font-synthesis: none;
                            text-rendering: optimizeLegibility;
                            -webkit-font-smoothing: antialiased;
                            -moz-osx-font-smoothing: grayscale;
                        }
                        
                        body {
                            margin: 0;
                            display: flex;
                            flex-direction: column;
                            place-items: center;
                            min-width: 320px;
                            min-height: 100vh;
                            padding: 20px;
                            position: relative;
                        }
                        
                        .balance-corner {
                            position: fixed;
                            top: 20px;
                            right: 20px;
                            font-size: 1em;
                            padding: 0.6em 1em;
                            background: #1a1a1a;
                            border-radius: 8px;
                            border: 1px solid #646cff;
                            z-index: 1000;
                            box-shadow: 0 2px 8px rgba(0,0,0,0.3);
                        }
                        
                        .balance-corner h2 {
                            margin: 0 0 0.2em 0;
                            font-size: 0.9em;
                            opacity: 0.8;
                        }
                        
                        .balance-corner div {
                            margin: 0;
                            font-weight: bold;
                            font-size: 1.1em;
                        }
                        
                        #root {
                            max-width: 1280px;
                            margin: 0 auto;
                            padding: 1rem;
                            text-align: center;
                        }
                        
                        h1 {
                            font-size: 2.5em;
                            line-height: 1.1;
                            margin-bottom: 0.5em;
                        }
                        
                        h2 {
                            font-size: 1.5em;
                            margin-bottom: 0.5em;
                        }
                        
                        .card {
                            padding: 1.5em;
                            margin: 0.8em 0;
                            background: #333;
                            border-radius: 8px;
                            border: 1px solid #444;
                        }
                        
                        .card.compact {
                            padding: 1em;
                            margin: 0.5em 0;
                        }
                        
                        button {
                            border-radius: 8px;
                            border: 1px solid transparent;
                            padding: 0.6em 1.2em;
                            font-size: 1em;
                            font-weight: 500;
                            font-family: inherit;
                            background-color: #007bff;
                            color: white;
                            cursor: pointer;
                            transition: background-color 0.25s;
                            margin: 0.5em;
                        }
                        
                        button:hover {
                            background-color: #0056b3;
                        }
                        
                        button:focus,
                        button:focus-visible {
                            outline: 4px auto -webkit-focus-ring-color;
                        }
                        
                        button:disabled {
                            opacity: 0.6;
                            cursor: not-allowed;
                            background-color: #666;
                        }
                        
                        button:disabled:hover {
                            background-color: #666;
                        }
                        
                        input[type="number"] {
                            padding: 0.6em 1.2em;
                            font-size: 1em;
                            border: 2px solid #ccc;
                            border-radius: 8px;
                            background-color: #333;
                            color: white;
                            margin: 0.5em;
                        }
                        
                        textarea {
                            width: 100%;
                            max-width: 600px;
                            padding: 15px;
                            font-size: 16px;
                            border: 2px solid #ccc;
                            border-radius: 8px;
                            background-color: #333;
                            color: white;
                            resize: vertical;
                            font-family: monospace;
                        }
                        
                        .balance {
                            font-size: 1.2em;
                            margin: 0.5em 0;
                            padding: 0.8em;
                            background: #1a1a1a;
                            border-radius: 8px;
                            border: 1px solid #646cff;
                            display: inline-block;
                            min-width: 200px;
                        }
                        
                        .balance h2 {
                            margin: 0 0 0.3em 0;
                            font-size: 1.1em;
                        }
                        
                        .balance div {
                            margin: 0;
                            font-weight: bold;
                        }
                        
                        .form-group {
                            margin: 1em 0;
                            display: flex;
                            flex-direction: column;
                            align-items: center;
                        }
                        
                        .success {
                            color: #28a745;
                            margin: 1em 0;
                            padding: 1em;
                            background: #1a1a1a;
                            border-radius: 8px;
                            border: 1px solid #28a745;
                        }
                        
                        .error {
                            color: #dc3545;
                            margin: 1em 0;
                            padding: 1em;
                            background: #1a1a1a;
                            border-radius: 8px;
                            border: 1px solid #dc3545;
                        }
                        
                        @media (prefers-color-scheme: light) {
                            :root {
                                color: #213547;
                                background-color: #ffffff;
                            }
                            
                            .card {
                                background: #f9f9f9;
                                border-color: #ddd;
                            }
                            
                            input[type="number"],
                            textarea {
                                background-color: #ffffff;
                                color: #213547;
                            }
                            
                            .balance {
                                background: #f0f0f0;
                            }
                            
                            .balance-corner {
                                background: #f0f0f0;
                                color: #213547;
                                border-color: #646cff;
                                box-shadow: 0 2px 8px rgba(0,0,0,0.1);
                            }
                            
                            .success {
                                background: #f0f0f0;
                            }
                            
                            .error {
                                background: #f0f0f0;
                            }
                        }
                    "#))
                }
            }
            body {
                div class="balance-corner" {
                    h2 { "Balance" }
                    div { 
                        span id="balance-value" { (balance) }
                        " sats"
                    }
                }
                div id="root" {
                    (content)
                }
                script {
                    (PreEscaped(r#"
                        async function refreshBalance() {
                            try {
                                const response = await fetch('/balance');
                                const data = await response.json();
                                const balanceElement = document.getElementById('balance-value');
                                if (balanceElement) {
                                    balanceElement.textContent = data.balance;
                                }
                            } catch (error) {
                                console.error('Failed to refresh balance:', error);
                            }
                        }
                        
                        function copyToClipboard(text) {
                            navigator.clipboard.writeText(text).then(() => {
                                const button = event.target;
                                const originalText = button.textContent;
                                button.textContent = 'Copied!';
                                setTimeout(() => {
                                    button.textContent = originalText;
                                }, 1000);
                            });
                        }
                        
                        async function completeMint(quoteId) {
                            const button = event.target;
                            const originalText = button.textContent;
                            
                            // Disable button and show loading state
                            button.disabled = true;
                            button.textContent = 'Checking Payment...';
                            
                            try {
                                const response = await fetch('/complete-mint', {
                                    method: 'POST',
                                    headers: {
                                        'Content-Type': 'application/x-www-form-urlencoded',
                                    },
                                    body: 'quote_id=' + encodeURIComponent(quoteId)
                                });
                                
                                if (response.ok) {
                                    // Show success message briefly then redirect to home
                                    button.textContent = 'Mint Completed!';
                                    setTimeout(() => {
                                        window.location.href = '/';
                                    }, 1500);
                                } else {
                                    // Handle error
                                    button.textContent = 'Payment Not Found - Try Again';
                                    button.disabled = false;
                                    setTimeout(() => {
                                        button.textContent = originalText;
                                    }, 3000);
                                }
                            } catch (error) {
                                console.error('Error completing mint:', error);
                                button.textContent = 'Error - Try Again';
                                button.disabled = false;
                                setTimeout(() => {
                                    button.textContent = originalText;
                                }, 3000);
                            }
                        }
                        
                        // Auto-refresh balance every 10 seconds
                        setInterval(refreshBalance, 10000);
                    "#))
                }
            }
        }
    }
}

async fn index(State(state): State<Arc<AppState>>) -> Html<String> {
    let balance = cashu::get_balance(&state.mint_url).await.unwrap_or(0);
    
    let content = html! {
        h1 { "SpillCash - Cashu Wallet" }
        
        div class="card compact" {
            h2 { "Mint Configuration" }
            p { "Connected to: " (state.mint_url) }
        }
        
        div class="card" {
            h2 { "Mint Ecash" }
            form action="/mint-quote" method="post" {
                div class="form-group" {
                    label for="mint-amount" { "Amount (sats):" }
                    input type="number" id="mint-amount" name="amount" placeholder="1000" min="1";
                }
                button type="submit" { "Get Lightning Invoice" }
            }
        }
        
        div class="card" {
            h2 { "Create Token" }
            form action="/token" method="post" {
                div class="form-group" {
                    label for="token-amount" { "Amount (sats):" }
                    input type="number" id="token-amount" name="amount" placeholder="100" min="1" required;
                }
                button type="submit" { "Create Token" }
            }
        }
    };
    
    Html(base_template("SpillCash - Cashu Wallet", content, balance).into_string())
}

async fn handle_mint(State(state): State<Arc<AppState>>, Form(form): Form<MintForm>) -> Result<Html<String>, StatusCode> {
    match cashu::mint(form.amount, &state.mint_url).await {
        Ok(_) => {
            let balance = cashu::get_balance(&state.mint_url).await.unwrap_or(0);
            let content = html! {
                h1 { "SpillCash - Cashu Wallet" }
                
                div class="success" {
                    h2 { "✓ Mint Successful!" }
                    p { "Minted " (form.amount.unwrap_or(1000)) " sats successfully." }
                }
                
                div class="card" {
                    h2 { "Mint Ecash" }
                    form action="/mint-quote" method="post" {
                        div class="form-group" {
                            label for="mint-amount" { "Amount (sats):" }
                            input type="number" id="mint-amount" name="amount" placeholder="1000" min="1";
                        }
                        button type="submit" { "Get Lightning Invoice" }
                    }
                }
                
                div class="card" {
                    h2 { "Create Token" }
                    form action="/token" method="post" {
                        div class="form-group" {
                            label for="token-amount" { "Amount (sats):" }
                            input type="number" id="token-amount" name="amount" placeholder="100" min="1" required;
                        }
                        button type="submit" { "Create Token" }
                    }
                }
                
                div {
                    a href="/" { "← Back to Home" }
                }
            };
            
            Ok(Html(base_template("SpillCash - Mint Success", content, balance).into_string()))
        }
        Err(e) => {
            let balance = cashu::get_balance(&state.mint_url).await.unwrap_or(0);
            let content = html! {
                h1 { "SpillCash - Cashu Wallet" }
                
                div class="error" {
                    h2 { "✗ Mint Failed" }
                    p { "Error: " (e.to_string()) }
                }
                
                div {
                    a href="/" { "← Back to Home" }
                }
            };
            
            Ok(Html(base_template("SpillCash - Mint Error", content, balance).into_string()))
        }
    }
}

async fn get_balance(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let balance = cashu::get_balance(&state.mint_url).await.unwrap_or(0);
    Json(json!({
        "balance": balance
    }))
}

async fn create_token(State(state): State<Arc<AppState>>, Form(form): Form<TokenForm>) -> Result<Html<String>, StatusCode> {
    match cashu::get_token(form.amount, &state.mint_url).await {
        Ok(token) => {
            let balance = cashu::get_balance(&state.mint_url).await.unwrap_or(0);
            let content = html! {
                h1 { "SpillCash - Cashu Wallet" }
                
                div class="success" {
                    h2 { "✓ Token Created!" }
                    p { "Created token for " (form.amount) " sats." }
                }
                
                div class="card" {
                    h2 { "Your Token" }
                    textarea readonly rows="8" {
                        (token)
                    }
                    button type="button" onclick={ "copyToClipboard('" (token) "')" } {
                        "Copy Token"
                    }
                }
                
                div {
                    a href="/" { "← Back to Home" }
                }
            };
            
            Ok(Html(base_template("SpillCash - Token Created", content, balance).into_string()))
        }
        Err(e) => {
            let balance = cashu::get_balance(&state.mint_url).await.unwrap_or(0);
            let content = html! {
                h1 { "SpillCash - Cashu Wallet" }
                
                div class="error" {
                    h2 { "✗ Token Creation Failed" }
                    p { "Error: " (e.to_string()) }
                }
                
                div {
                    a href="/" { "← Back to Home" }
                }
            };
            
            Ok(Html(base_template("SpillCash - Token Error", content, balance).into_string()))
        }
    }
}

async fn create_mint_quote_handler(State(state): State<Arc<AppState>>, Form(form): Form<MintForm>) -> Result<Html<String>, StatusCode> {
    match cashu::create_mint_quote(form.amount, &state.mint_url).await {
        Ok(quote) => {
            let balance = cashu::get_balance(&state.mint_url).await.unwrap_or(0);
            let content = html! {
                h1 { "SpillCash - Cashu Wallet" }
                
                div class="success" {
                    h2 { "Lightning Invoice Generated" }
                    p { "Amount: " (form.amount.unwrap_or(1000)) " sats" }
                    p { "Please pay the Lightning invoice below:" }
                }
                
                div class="card" {
                    h2 { "Lightning Invoice" }
                    textarea readonly rows="6" id="invoice" {
                        (quote.request)
                    }
                    button type="button" onclick={ "copyToClipboard('" (quote.request) "')" } {
                        "Copy Invoice"
                    }
                }
                
                div class="card" {
                    h2 { "Complete Mint" }
                    p { "After paying the invoice, click the button below to complete the mint:" }
                    button type="button" onclick={ "completeMint('" (quote.id) "')" } {
                        "Complete Mint"
                    }
                    p style="margin-top: 1em; font-size: 0.9em; color: #888;" { 
                        "This button will check if your payment has been received and complete the minting process." 
                    }
                }
                
                div {
                    a href="/" { "← Back to Home" }
                }
            };
            
            Ok(Html(base_template("SpillCash - Lightning Invoice", content, balance).into_string()))
        }
        Err(e) => {
            let balance = cashu::get_balance(&state.mint_url).await.unwrap_or(0);
            let content = html! {
                h1 { "SpillCash - Cashu Wallet" }
                
                div class="error" {
                    h2 { "✗ Quote Creation Failed" }
                    p { "Error: " (e.to_string()) }
                }
                
                div {
                    a href="/" { "← Back to Home" }
                }
            };
            
            Ok(Html(base_template("SpillCash - Quote Error", content, balance).into_string()))
        }
    }
}

async fn complete_mint_handler(State(state): State<Arc<AppState>>, Form(form): Form<CompleteMintForm>) -> Result<Html<String>, StatusCode> {
    match cashu::complete_mint(&form.quote_id, &state.mint_url).await {
        Ok(minted_amount) => {
            let balance = cashu::get_balance(&state.mint_url).await.unwrap_or(0);
            let content = html! {
                h1 { "SpillCash - Cashu Wallet" }
                
                div class="success" {
                    h2 { "✓ Mint Completed!" }
                    p { "Successfully minted " (minted_amount) " sats." }
                }
                
                div class="card" {
                    h2 { "Mint Ecash" }
                    form action="/mint-quote" method="post" {
                        div class="form-group" {
                            label for="mint-amount" { "Amount (sats):" }
                            input type="number" id="mint-amount" name="amount" placeholder="1000" min="1";
                        }
                        button type="submit" { "Get Lightning Invoice" }
                    }
                }
                
                div class="card" {
                    h2 { "Create Token" }
                    form action="/token" method="post" {
                        div class="form-group" {
                            label for="token-amount" { "Amount (sats):" }
                            input type="number" id="token-amount" name="amount" placeholder="100" min="1" required;
                        }
                        button type="submit" { "Create Token" }
                    }
                }
                
                div {
                    a href="/" { "← Back to Home" }
                }
            };
            
            Ok(Html(base_template("SpillCash - Mint Complete", content, balance).into_string()))
        }
        Err(e) => {
            let balance = cashu::get_balance(&state.mint_url).await.unwrap_or(0);
            let content = html! {
                h1 { "SpillCash - Cashu Wallet" }
                
                div class="error" {
                    h2 { "✗ Mint Failed" }
                    p { "Error: " (e.to_string()) }
                    p { "The invoice may not have been paid yet, or there was an error processing the payment." }
                }
                
                div {
                    a href="/" { "← Back to Home" }
                }
            };
            
            Ok(Html(base_template("SpillCash - Mint Error", content, balance).into_string()))
        }
    }
}
