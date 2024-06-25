use tokio;

#[tokio::test]
pub async fn test_get_issues() {
    use crate::client::client::RedmineClient;
    use crate::tests::utils::UrlApik;
    use dotenv::dotenv;

    dotenv().ok();

    let urlapik = UrlApik::new();
    if urlapik.any_empty() {
        eprintln!("REDMINE_URL and REDMINE_API_KEY not found. Please set them on environment variables before running the test.");
    }

    let client = RedmineClient::new(&urlapik.url, &urlapik.apik);

    match client.get_issues().await {
        Ok(issues_result) => match &issues_result.issues {
            Some(issues) => {
                for issue in issues {
                    println!("Issue: {:?} - {:?}", issue.id, issue.subject);
                }
            }
            None => println!("Found no issues at all."),
        },
        Err(err) => {
            eprintln!("Error fetching issues: {}", err);
        }
    }
}

// pub async fn test_get_single_issue() {
//     use crate::client::client::RedmineClient;
//     use dotenv::dotenv;
//     use std::env;
// }
