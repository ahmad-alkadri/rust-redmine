use tokio;

#[tokio::test]
pub async fn test_get_issues() {
    use crate::client::client::RedmineClient;
    use crate::tests::utils::UrlApik;
    use dotenv::dotenv;

    // Load environment variables
    dotenv().ok();

    // Create UrlApik instance and check for required environment variables
    let urlapik = UrlApik::new();
    assert!(!urlapik.any_empty(), "REDMINE_URL and REDMINE_API_KEY not found. Please set them in environment variables before running the test.");

    // Create RedmineClient instance
    let client = RedmineClient::new(&urlapik.url, &urlapik.apik, None, None);

    // Fetch issues and handle result
    let issues_result = client
        .get_issues(None, None)
        .await
        .expect("Failed to fetch issues");

    if let Some(issues) = issues_result.issues {
        for issue in issues {
            println!("Issue: {:?} - {:?}", issue.id, issue.subject);
        }
    } else {
        println!("Found no issues at all.");
    }
}
