use reqwest::Client;

pub struct RedmineClient {
    pub base_url: String,
    pub api_key: String,
    pub client: Client,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

impl RedmineClient {
    pub fn new(base_url: &str, api_key: &str, limit: Option<i32>, offset: Option<i32>) -> Self {
        RedmineClient {
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
            client: Client::new(),
            limit,
            offset,
        }
    }

    pub fn get_pagination_clause(&self, limit: Option<i32>, offset: Option<i32>) -> String {
        let mut clause: String = "".to_string();

        let limit_value = limit.or(self.limit);
        let offset_value = offset.or(self.offset);

        if let Some(limit) = limit_value {
            if limit > -1 {
                clause = clause + &format!("&limit={}", limit);
            }
        }
        if let Some(offset) = offset_value {
            if offset > -1 {
                clause = clause + &format!("&offset={}", offset);
            }
        }

        clause
    }
}
