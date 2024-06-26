use crate::{
    client::client::RedmineClient,
    fields::projects::{Project, ProjectsResult},
};

impl RedmineClient {
    pub async fn get_projects(&self) -> Result<Vec<Project>, Box<dyn std::error::Error>> {
        let url: String = format!(
            "{}/projects.json?key={}{}",
            self.base_url,
            self.api_key,
            self.get_pagination_clause()
        );
        let response: reqwest::Response = self.client.get(&url).send().await?;

        // Check the status code first
        if !response.status().is_success() {
            let status: reqwest::StatusCode = response.status();
            return Err(format!("Failed to fetch projects: Status code {}", status).into());
        }

        // Attempt to deserialize the response using match
        let response_text: String = response.text().await?;
        let project_result: ProjectsResult = match serde_json::from_str(&response_text) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Error decoding response: {}", e);
                return Err(Box::new(e));
            }
        };
        Ok(project_result
            .projects
            .expect("`projects` field not found."))
    }
}
