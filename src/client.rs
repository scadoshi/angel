use crate::{action::Action, config::Config, token::Token};
use reqwest::{blocking::Client, StatusCode};
use serde_json::Value;

pub struct AuthenticatedClient {
    config: Config,
    token: Token,
}

impl AuthenticatedClient {
    pub fn new() -> anyhow::Result<Self> {
        let config = Config::new()?;
        let token = config.get_token()?;
        Ok(Self { config, token })
    }
    pub fn access_token(&mut self) -> anyhow::Result<&str> {
        if self.token.is_expired() {
            self.token = self.config.get_token()?;
            Ok(&self.token.access_token)
        } else {
            Ok(&self.token.access_token)
        }
    }
    pub fn post_action(&mut self, action: Action) -> anyhow::Result<Value> {
        let response = Client::new()
            .post(self.config.url.clone().join("api/actions")?)
            .bearer_auth(self.access_token()?)
            .header("Content-Type", "application/json")
            .json(&[serde_json::to_value(action)?])
            .send()?;
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => Ok(response.json()?),
            _ => Err(anyhow::anyhow!(
                "Post failed with status code: {} and response: {}",
                response.status(),
                response.json::<Value>()?
            )),
        }
    }
}
