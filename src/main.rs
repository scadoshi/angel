mod action;
mod config;
mod token;

use crate::{action::Action, config::Config, token::Token};
use chrono::Utc;
use reqwest::{blocking::Client, StatusCode};

fn main() -> anyhow::Result<()> {
    let config = Config::new()?;
    let result = Client::new()
        .post(config.url.clone().join("auth/token")?)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .basic_auth(config.client_id, Some(config.client_secret))
        .body("grant_type=client_credentials&scope=all")
        .send()?;
    let token: Token = serde_json::from_str(&result.text()?)?;
    println!("✔  Authenticated");
    let action = Action::new(
        "Rust App",
        Utc::now().naive_utc(),
        "<b>bold note</b>",
        "Note Added via API",
        3132,
    );
    let result = Client::new()
        .post(config.url.clone().join("api/actions")?)
        .bearer_auth(token.access_token)
        .header("Content-Type", "application/json")
        .json(&[serde_json::to_value(action)?])
        .send()?;
    match result.status() {
        StatusCode::OK | StatusCode::CREATED => println!("✔  Action post succeeded"),
        _ => {
            println!("✘  Action post failed");
            println!("- Status: {:#?}", result.status());
            println!("- Response: {:?}", result.text());
        }
    }
    Ok(())
}
