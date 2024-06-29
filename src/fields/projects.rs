use crate::fields::ids::IdName;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<IdName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub identifier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherit_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_ids: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_module_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_values: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectResult {
    pub project: Option<Project>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectsResult {
    pub projects: Option<Vec<Project>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectCreationRequest {
    pub project: Project,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomFieldValue {
    pub id: i32,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectToCreate {
    pub name: String,
    pub identifier: String,
    pub description: String,
    pub is_public: bool,
    pub parent_id: i32,
    pub inherit_members: bool,
    pub tracker_ids: Vec<i32>,
    pub enabled_module_names: Vec<String>,
    pub custom_field_values: std::collections::HashMap<String, CustomFieldValue>,
}
