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
}
