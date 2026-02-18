use reqwest::blocking::Client;

use crate::token::{RawToken, Token};

pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub url: url::Url,
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        dotenvy::dotenv()?;
        let client_id = std::env::var("CLIENT_ID")?;
        let client_secret = std::env::var("CLIENT_SECRET")?;
        let url = url::Url::parse(&std::env::var("URL")?)?;
        Ok(Self {
            client_id,
            client_secret,
            url,
        })
    }
    pub fn get_token(&self) -> anyhow::Result<Token> {
        let result = Client::new()
            .post(self.url.clone().join("auth/token")?)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .body("grant_type=client_credentials&scope=all")
            .send()?;
        let raw_token: RawToken = serde_json::from_str(&result.text()?)?;
        let token = Token::from(raw_token);
        Ok(token)
    }
}
