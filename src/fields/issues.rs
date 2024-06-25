use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueRequest {
    pub(crate) pubissue: Issue,
    pub(crate) issue: Issue,
}

#[derive(Deserialize, Debug)]
pub struct IssueResult {
    pub issue: Option<Issue>,
}

#[derive(Deserialize, Debug)]
pub struct IssuesResult {
    pub issues: Option<Vec<Issue>>,
    pub total_count: Option<u32>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JournalDetails {
    property: Option<String>,
    name: Option<String>,
    old_value: Option<String>,
    new_value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Journal {
    id: Option<i32>,
    user: Option<IdName>,
    notes: Option<String>,
    created_on: Option<String>,
    details: Option<Vec<JournalDetails>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Issue {
    pub id: Option<i32>,
    pub subject: Option<String>,
    pub description: Option<String>,
    pub project_id: Option<String>,
    pub project: Option<IdName>,
    pub tracker_id: Option<i32>,
    pub tracker: Option<IdName>,
    pub parent_issue_id: Option<i32>,
    pub parent: Option<Id>,
    pub status_id: Option<i32>,
    pub status: Option<IdName>,
    pub priority_id: Option<i32>,
    pub priority: Option<IdName>,
    pub author: Option<IdName>,
    pub fixed_version: Option<IdName>,
    pub assigned_to: Option<IdName>,
    pub assigned_to_id: Option<i32>,
    pub category: Option<IdName>,
    pub category_id: Option<String>,
    pub notes: Option<String>,
    pub status_date: Option<String>,
    pub created_on: Option<String>,
    pub updated_on: Option<String>,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    pub closed_on: Option<String>,
    pub custom_fields: Option<Vec<CustomField>>,
    pub uploads: Option<Vec<Upload>>,
    pub done_ratio: Option<f32>,
    pub estimated_hours: Option<f32>,
    pub journals: Option<Vec<Journal>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdName {
    id: Option<i32>,
    name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Id {
    id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomField {
    id: Option<i32>,
    name: Option<String>,
    description: Option<String>,
    multiple: bool,
    value: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Upload {
    id: Option<String>,
    filename: Option<String>,
    filesize: Option<u64>,
    content_type: Option<String>,
    description: Option<String>,
    token: Option<String>,
}

#[derive(Debug)]
pub struct IssueFilter {
    pub project_id: Option<String>,
    pub subproject_id: Option<String>,
    pub tracker_id: Option<String>,
    pub status_id: Option<String>,
    pub assigned_to_id: Option<String>,
    pub updated_on: Option<String>,
    pub extra_filters: Option<std::collections::HashMap<String, String>>,
}
