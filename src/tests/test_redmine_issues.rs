use tokio;

#[tokio::test]
pub async fn test_get_issues() {
    use crate::client::client::RedmineClient;
    use dotenv::dotenv;
    use std::env;

    dotenv().ok();

    let redmine_url = match env::var("REDMINE_URL") {
        Ok(url) => url,
        Err(_) => {
            eprintln!("Environment variable REDMINE_URL is not set. Please set it before running the test.");
            return;
        }
    };

    let api_key = match env::var("REDMINE_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Environment variable REDMINE_API_KEY is not set. Please set it before running the test.");
            return;
        }
    };

    let client = RedmineClient::new(&redmine_url, &api_key);

    match client.get_issues().await {
        Ok(issues) => {
            for issue in issues {
                println!("Issue: {} - {}", issue.id, issue.subject);
            }
        }
        Err(err) => {
            eprintln!("Error fetching issues: {}", err);
        }
    }
}
