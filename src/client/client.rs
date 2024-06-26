use reqwest::Client;

pub struct RedmineClient {
    pub base_url: String,
    pub api_key: String,
    pub client: Client,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl RedmineClient {
    pub fn new(base_url: &str, api_key: &str, limit: Option<i64>, offset: Option<i64>) -> Self {
        RedmineClient {
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
            client: Client::new(),
            limit,
            offset,
        }
    }

    pub fn get_pagination_clause(&self) -> String {
        let mut clause: String = "".to_string();
        if let Some(limit) = self.limit {
            if limit > -1 {
                clause = clause + &format!("&limit={}", limit)
            }
        }
        if let Some(offset) = self.offset {
            if offset > -1 {
                clause = clause + &format!("&offset={}", offset)
            }
        }
        return clause;
    }
}
