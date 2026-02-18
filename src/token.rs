use jiff::{civil::DateTime, tz::TimeZone, Timestamp};
use std::time::Duration;

#[derive(Debug, serde::Deserialize)]
pub struct RawToken {
    //pub scope: String,
    //pub token_type: String,
    pub access_token: String,
    pub expires_in: u64,
    //pub refresh_token: String,
    //pub id_token: String,
}

#[derive(Debug)]
pub struct Token {
    pub access_token: String,
    pub expiration: DateTime,
}

impl From<RawToken> for Token {
    fn from(value: RawToken) -> Self {
        let now = Timestamp::now().to_zoned(TimeZone::UTC).datetime();
        let seconds = Duration::from_secs(value.expires_in.saturating_sub(300));
        let expiration = now.saturating_add(seconds);
        Self {
            access_token: value.access_token,
            expiration,
        }
    }
}

impl Token {
    pub fn is_expired(&self) -> bool {
        let now = Timestamp::now().to_zoned(TimeZone::UTC).datetime();
        now > self.expiration
    }
}
