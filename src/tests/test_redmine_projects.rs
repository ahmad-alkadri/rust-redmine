use tokio;

#[tokio::test]
pub async fn test_create_get_delete_project() {
    use crate::client::client::RedmineClient;
    use crate::fields::projects::Project;
    use crate::tests::utils::UrlApik;
    use dotenv::dotenv;

    // Load environment variables
    dotenv().ok();

    // Create UrlApik instance and check for required environment variables
    let urlapik = UrlApik::new();
    assert!(!urlapik.any_empty(), "REDMINE_URL and REDMINE_API_KEY not found. Please set them in environment variables before running the test.");

    // Create RedmineClient instance
    let client: RedmineClient = RedmineClient::new(&urlapik.url, &urlapik.apik, None, None);

    // Prepare the project that you want to create
    let project = Project {
        name: Some("Test Project X".to_string()),
        identifier: "test-project-x".to_string(),
        is_public: Some(false),
        description: Some("This is the test project X".to_string()),
        id: None,
        parent: None,
        parent_id: None,
        inherit_members: None,
        tracker_ids: None,
        created_on: None,
        updated_on: None,
        enabled_module_names: None,
        custom_field_values: None,
    };

    let project_res = client.create_project(project).await;
    let project_req = client.get_project("test-project-x".to_string()).await;
    let project_del = client.delete_project("test-project-x".to_string()).await;

    assert!(
        project_res.is_ok(),
        "Failed to create project: {:?}",
        project_res.err().unwrap()
    );

    assert!(
        project_req.is_ok(),
        "Failed to get the project that was just created: {:?}",
        project_req.err().unwrap()
    );

    assert!(
        project_del.is_ok(),
        "Failed to delete the target project: {:?}",
        project_del.err().unwrap()
    );
}

#[tokio::test]
pub async fn test_get_projects() {
    use crate::client::client::RedmineClient;
    use crate::tests::utils::UrlApik;
    use dotenv::dotenv;

    // Load environment variables
    dotenv().ok();

    // Create UrlApik instance and check for required environment variables
    let urlapik = UrlApik::new();
    assert!(!urlapik.any_empty(), "REDMINE_URL and REDMINE_API_KEY not found. Please set them in environment variables before running the test.");

    // Create RedmineClient instance
    let client: RedmineClient = RedmineClient::new(&urlapik.url, &urlapik.apik, None, None);

    let projects = client.get_projects(None, None).await;

    assert!(
        projects.is_ok(),
        "Failed to delete the target project: {:?}",
        projects.err().unwrap()
    );
}
