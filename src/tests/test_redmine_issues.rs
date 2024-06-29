use tokio;

#[tokio::test]
pub async fn test_create_get_delete_issue() {
    use crate::client::client::RedmineClient;
    use crate::tests::utils::{example_issue, example_issue_filter, example_project, UrlApik};
    use dotenv::dotenv;
    use uuid::Uuid;

    // Load environment variables
    dotenv().ok();

    let uuid = Uuid::new_v4();

    // Create UrlApik instance and check for required environment variables
    let urlapik = UrlApik::new();
    assert!(!urlapik.any_empty(), "REDMINE_URL and REDMINE_API_KEY not found. Please set them in environment variables before running the test.");

    // Create RedmineClient instance
    let client = RedmineClient::new(&urlapik.url, &urlapik.apik, None, None);

    // Create the project to attach the issue
    let unique_project_id = uuid.to_string();
    let project_res = client
        .create_project(example_project(
            unique_project_id.clone(),
            unique_project_id.clone(),
        ))
        .await;

    // If project creation is failed:
    if !project_res.is_ok() {
        // Check first why does it fail. Expected reason would be because the project already exists
        let reason = format!("{:?}", project_res.err().unwrap());
        let reason_expected = "Identifier has already been taken".to_string();
        assert!(
            reason.contains(&reason_expected),
            "Error when trying to create the target project to attach the issue: {:?}",
            reason
        );
    }

    let unique_subject = uuid.to_string();

    let issue = example_issue(unique_subject.clone(), unique_project_id.clone());
    let filter_issue = example_issue_filter(unique_project_id);

    let issue_res = client.create_issue(issue).await;
    let issue_in_project: Result<Option<Vec<crate::fields::issues::Issue>>, reqwest::Error> =
        client.get_issues_by_filter(&filter_issue).await;

    assert!(
        issue_res.is_ok(),
        "Failed creating new issue: {:?}",
        issue_res.err().unwrap()
    );

    assert!(
        issue_in_project.is_ok(),
        "Found error when trying to find issue in the target project: {:?}",
        issue_in_project.err().unwrap()
    ); // 1. there is no error

    let issues = issue_in_project.unwrap(); // unwrap the Result
    assert!(issues.is_some()); // 2. there is a Vec<crate::fields::issues::Issue>, not None

    let issues_vec = issues.unwrap(); // unwrap the Option
    assert!(
        !issues_vec.is_empty(),
        "Not found any issues in the target project."
    ); // make sure the vec is not empty

    let issue = issues_vec
        .into_iter()
        .find(|issue| issue.subject == Some(unique_subject.clone()));
    assert!(issue.is_some());
}

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
    let issues_result = client.get_issues(None, None).await;

    assert!(
        issues_result.is_ok(),
        "Failed to get issues: {:?}",
        issues_result.err().unwrap()
    );
}
