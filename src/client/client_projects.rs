use std::error::Error;

use reqwest::{Request, Response, StatusCode};

use crate::{
    client::client::RedmineClient,
    fields::{
        errors::ErrorsResult,
        projects::{Project, ProjectCreationRequest, ProjectResult, ProjectsResult},
    },
};

impl RedmineClient {
    pub async fn delete_project(&self, project_id: String) -> Result<(), Box<dyn Error>> {
        let url = format!(
            "{}/projects/{}.json?key={}",
            self.base_url, project_id, self.api_key
        );

        let req: Request = self.client.delete(url).build()?;

        let res: Response = self.client.execute(req).await?;

        if res.status() == StatusCode::NOT_FOUND {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Not Found",
            )));
        }

        if res.status() != StatusCode::NO_CONTENT {
            let er: ErrorsResult = res.json().await?;
            let error_message = er.errors.join("\n");
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                error_message,
            )));
        }

        Ok(())
    }

    pub async fn create_project(&self, project: Project) -> Result<ProjectResult, Box<dyn Error>> {
        let ir = ProjectCreationRequest { project };
        let s = serde_json::to_string(&ir)?;

        let url = format!("{}/projects.json?key={}", self.base_url, self.api_key);
        let req = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .body(s)
            .build()?;

        let res: Response = self.client.execute(req).await?;

        let statuscode = res.status();
        if statuscode != reqwest::StatusCode::CREATED {
            let er: ErrorsResult = res.json().await?;
            let error_message = er.errors.join("\n");
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                error_message,
            )));
        }

        let r: ProjectResult = res.json().await?;

        Ok(r)
    }

    pub async fn get_project(&self, project_id: String) -> Result<ProjectResult, Box<dyn Error>> {
        let url = format!(
            "{}/projects/{}.json?key={}",
            self.base_url, project_id, self.api_key,
        );

        let response = self.client.get(&url).send().await?;

        // Check the status code first
        if !response.status().is_success() {
            let status = response.status();
            return Err(format!("Failed to fetch projects: Status code {}", status).into());
        }

        // Attempt to deserialize the response using match
        let response_text = response.text().await?;

        let project_result: ProjectResult = match serde_json::from_str(&response_text) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Error decoding response: {}", e);
                return Err(Box::new(e));
            }
        };

        Ok(project_result)
    }

    pub async fn get_projects(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<Project>, Box<dyn Error>> {
        let url: String = format!(
            "{}/projects.json?key={}{}",
            self.base_url,
            self.api_key,
            self.get_pagination_clause(limit, offset)
        );
        let response: reqwest::Response = self.client.get(&url).send().await?;

        // Check the status code first
        if !response.status().is_success() {
            let status: reqwest::StatusCode = response.status();
            return Err(format!("Failed to fetch projects: Status code {}", status).into());
        }

        // Attempt to deserialize the response using match
        let response_text: String = response.text().await?;
        let projects_result: ProjectsResult = match serde_json::from_str(&response_text) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Error decoding response: {}", e);
                return Err(Box::new(e));
            }
        };
        Ok(projects_result
            .projects
            .expect("`projects` field not found."))
    }
}
