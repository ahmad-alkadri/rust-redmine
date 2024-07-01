use tokio;

#[tokio::test]
pub async fn test_create_get_delete_issue() {
    use crate::client::client::RedmineClient;
    use crate::tests::utils::{example_issue, example_issue_filter, example_project, UrlApik};
    use dotenv::dotenv;
    use uuid::Uuid;

    // Load environment variables
    dotenv().ok();

    let uuid_issue = Uuid::new_v4();
    let uuid_project = Uuid::new_v4();

    // Create UrlApik instance and check for required environment variables
    let urlapik = UrlApik::new();
    assert!(!urlapik.any_empty(), "REDMINE_URL and REDMINE_API_KEY not found. Please set them in environment variables before running the test.");

    // Create RedmineClient instance
    let client = RedmineClient::new(&urlapik.url, &urlapik.apik, None, None);

    // Create the project to attach the issue
    let unique_project_id = uuid_project.to_string();
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

    let unique_subject = uuid_issue.to_string();

    // Try creating an Issue
    let issue_res = client
        .create_issue(example_issue(
            unique_subject.clone(),
            None,
            unique_project_id.clone(),
        ))
        .await;

    // Try getting the issue by taking all the issues in the project
    let issue_in_project = client
        .get_issues_by_filter(&example_issue_filter(unique_project_id.clone()))
        .await;

    assert!(
        issue_res.is_ok(),
        "Failed creating new issue: {:?}",
        issue_res.err().unwrap()
    );

    let issueres = issue_res.ok().unwrap();
    assert!(
        issueres.issue.is_some(),
        "Can't read the newly created issue; check with the Redmine",
    );
    let mut issue_id = 1;
    if let Some(issue_created) = issueres.issue {
        if let Some(issue_created_id) = issue_created.id {
            issue_id = issue_created_id;
        }
    }

    // Try updating the issue
    let issue_updated = client
        .update_issue(
            issue_id,
            example_issue(
                "Updated subject".to_string(),
                None,
                unique_project_id.clone(),
            ),
        )
        .await;

    // Try deleting the issue
    let issue_deleted = client.delete_issue(issue_id).await;

    // Try deleting the temporary project to which the issue was attached to
    let project_deleted = client.delete_project(unique_project_id.clone()).await;

    // ASSERTIONS BLOCK --- //
    // Assert there's no error when trying to find issue in the target project
    assert!(
        issue_in_project.is_ok(),
        "Found error when trying to find issue in the target project: {:?}",
        issue_in_project.err().unwrap()
    ); // 1. there is no error

    // Assert that there's indeed issues in the project, not None
    let issues = issue_in_project.unwrap();
    assert!(issues.is_some());

    // Assert that the newly created issue is found within the project
    let issues_vec = issues.unwrap();
    assert!(
        !issues_vec.is_empty(),
        "Not found any issues in the target project."
    );
    let issue = issues_vec
        .into_iter()
        .find(|issue| issue.subject == Some(unique_subject.clone()));
    assert!(issue.is_some());

    // Assert that the issue update works
    assert!(
        issue_updated.is_ok(),
        "Failed in updating issue id {:?}: {:?}",
        issue_id,
        issue_updated.err().unwrap()
    );

    // Assert that the issue deletion works
    assert!(
        issue_deleted.is_ok(),
        "Failed in deleting issue id {:?}: {:?}:",
        issue_id,
        issue_deleted.err().unwrap()
    );

    // Assert that the project deletion works
    assert!(
        project_deleted.is_ok(),
        "Failed in deleting temporary project with id {:?}: {:?}",
        unique_project_id,
        project_deleted.err().unwrap()
    );
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
