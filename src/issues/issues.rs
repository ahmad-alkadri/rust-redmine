use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueRequest {
    pubissue: Issue,
    pub(crate) issue: Issue,
}

#[derive(Deserialize, Debug)]
pub struct IssueResult {
    pub issue: Issue,
}

#[derive(Deserialize, Debug)]
pub struct IssuesResult {
    pub issues: Vec<Issue>,
    pub total_count: u32,
    pub offset: u32,
    pub limit: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JournalDetails {
    property: String,
    name: String,
    old_value: String,
    new_value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Journal {
    id: i32,
    user: Option<IdName>,
    notes: String,
    created_on: String,
    details: Vec<JournalDetails>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Issue {
    pub id: i32,
    pub subject: String,
    pub description: String,
    pub project_id: String,
    pub project: Option<IdName>,
    pub tracker_id: i32,
    pub tracker: Option<IdName>,
    pub parent_issue_id: Option<i32>,
    pub parent: Option<Id>,
    pub status_id: i32,
    pub status: Option<IdName>,
    pub priority_id: Option<i32>,
    pub priority: Option<IdName>,
    pub author: Option<IdName>,
    pub fixed_version: Option<IdName>,
    pub assigned_to: Option<IdName>,
    pub assigned_to_id: Option<i32>,
    pub category: Option<IdName>,
    pub category_id: Option<String>,
    pub notes: String,
    pub status_date: String,
    pub created_on: String,
    pub updated_on: String,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    pub closed_on: Option<String>,
    pub custom_fields: Option<Vec<CustomField>>,
    pub uploads: Option<Vec<Upload>>,
    pub done_ratio: f32,
    pub estimated_hours: f32,
    pub journals: Option<Vec<Journal>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdName {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Id {
    id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomField {
    id: i32,
    name: String,
    description: String,
    multiple: bool,
    value: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Upload {
    id: String,
    filename: String,
    filesize: u64,
    content_type: String,
    description: Option<String>,
    token: String,
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
