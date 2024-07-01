use crate::fields::ids::IdName;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubissue: Option<Issue>,
    pub issue: Issue,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct JournalDetails {
    property: Option<String>,
    name: Option<String>,
    old_value: Option<String>,
    new_value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Journal {
    id: Option<i32>,
    user: Option<IdName>,
    notes: Option<String>,
    created_on: Option<String>,
    details: Option<Vec<JournalDetails>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Issue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<IdName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker: Option<IdName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_issue_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<IdName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<IdName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<IdName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_version: Option<IdName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<IdName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<IdName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploads: Option<Vec<Upload>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done_ratio: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_hours: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journals: Option<Vec<Journal>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Id {
    id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomField {
    id: Option<i32>,
    name: Option<String>,
    description: Option<String>,
    multiple: bool,
    value: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Upload {
    id: Option<String>,
    filename: Option<String>,
    filesize: Option<u64>,
    content_type: Option<String>,
    description: Option<String>,
    token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct IssueFilter {
    pub project_id: Option<String>,
    pub subproject_id: Option<String>,
    pub tracker_id: Option<String>,
    pub status_id: Option<String>,
    pub assigned_to_id: Option<String>,
    pub updated_on: Option<String>,
    pub extra_filters: Option<std::collections::HashMap<String, String>>,
}
