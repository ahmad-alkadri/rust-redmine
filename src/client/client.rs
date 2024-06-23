use reqwest::Client;

pub struct RedmineClient {
    pub base_url: String,
    pub api_key: String,
    pub client: Client,
}

impl RedmineClient {
    pub fn new(base_url: &str, api_key: &str) -> Self {
        RedmineClient {
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
            client: Client::new(),
        }
    }
}
