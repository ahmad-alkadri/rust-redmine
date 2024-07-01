use crate::{
    client::client::RedmineClient,
    fields::{
        errors::ErrorsResult,
        issues::{Issue, IssueFilter, IssueRequest, IssueResult, IssuesResult},
    },
};
use reqwest::{Error, Response, StatusCode};

impl RedmineClient {
    pub async fn get_issues(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<IssuesResult, Box<dyn std::error::Error>> {
        let url: String = format!(
            "{}/issues.json?key={}{}",
            self.base_url,
            self.api_key,
            self.get_pagination_clause(limit, offset),
        );
        let response: reqwest::Response = self.client.get(&url).send().await?;

        // Check the status code first
        if !response.status().is_success() {
            let status: reqwest::StatusCode = response.status();
            return Err(format!("Failed to fetch issues: Status code {}", status).into());
        }

        // Attempt to deserialize the response using match
        let response_text: String = response.text().await?;
        let issues_result: IssuesResult = match serde_json::from_str(&response_text) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Error decoding response: {}", e);
                return Err(Box::new(e));
            }
        };

        Ok(issues_result)
    }

    pub async fn get_issue(&self, id: &str) -> Result<Option<Issue>, Error> {
        let url: String = format!("{}/issues/{}.json?key={}", self.base_url, id, self.api_key);
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<IssueResult>()
            .await?;
        Ok(response.issue)
    }

    pub async fn get_issues_by_query(&self, query_id: &str) -> Result<Option<Vec<Issue>>, Error> {
        let url = format!(
            "{}/issues.json?query_id={}&key={}",
            self.base_url, query_id, self.api_key
        );
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<IssuesResult>()
            .await?;
        Ok(response.issues)
    }

    pub async fn get_issues_by_filter(
        &self,
        filter: &IssueFilter,
    ) -> Result<Option<Vec<Issue>>, Error> {
        let mut query_params = vec![("key", self.api_key.clone())];

        if let Some(project_id) = &filter.project_id {
            query_params.push(("project_id", project_id.clone()));
        }
        if let Some(subproject_id) = &filter.subproject_id {
            query_params.push(("subproject_id", subproject_id.clone()));
        }
        if let Some(tracker_id) = &filter.tracker_id {
            query_params.push(("tracker_id", tracker_id.clone()));
        }
        if let Some(status_id) = &filter.status_id {
            query_params.push(("status_id", status_id.clone()));
        }
        if let Some(assigned_to_id) = &filter.assigned_to_id {
            query_params.push(("assigned_to_id", assigned_to_id.clone()));
        }
        if let Some(updated_on) = &filter.updated_on {
            query_params.push(("updated_on", updated_on.clone()));
        }
        if let Some(extra_filters) = &filter.extra_filters {
            for (key, value) in extra_filters {
                query_params.push((key, value.clone()));
            }
        }

        let formatted_query_params: String = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&");

        let url: String = format!("{}/issues.json?{}", self.base_url, formatted_query_params);

        let response: IssuesResult = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<IssuesResult>()
            .await?;

        Ok(response.issues)
    }

    pub async fn create_issue(
        &self,
        issue: Issue,
    ) -> Result<IssueResult, Box<dyn std::error::Error>> {
        let ir = IssueRequest {
            pubissue: None,
            issue: issue.clone(),
        };
        let s = serde_json::to_string(&ir)?;

        let url = format!("{}/issues.json?key={}", self.base_url, self.api_key);
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

        let r: IssueResult = res.json().await?;

        Ok(r)
    }

    pub async fn update_issue(
        &self,
        id: i32,
        issue: Issue,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/issues/{}.json?key={}", self.base_url, id, self.api_key);
        let ir = IssueRequest {
            pubissue: None,
            issue,
        };
        let s = serde_json::to_string(&ir)?;
        let req = self
            .client
            .put(&url)
            .header("Content-Type", "application/json")
            .body(s)
            .build()?;

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

    pub async fn delete_issue(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/issues/{}.json?key={}", self.base_url, id, self.api_key);

        self.client.delete(&url).send().await?;
        Ok(())
    }
}
