use std::env;

pub struct UrlApik {
    pub url: String,
    pub apik: String,
}

impl UrlApik {
    pub fn new() -> UrlApik {
        UrlApik {
            url: env::var("REDMINE_URL").unwrap_or_else(|_| "".to_string()),
            apik: env::var("REDMINE_API_KEY").unwrap_or_else(|_| "".to_string()),
        }
    }

    pub fn any_empty(&self) -> bool {
        self.url.is_empty() || self.apik.is_empty()
    }
}
