use crate::fields::issues::IdName;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: u64,
    pub parent: IdName,
    pub name: String,
    pub identifier: String,
    pub description: Option<String>,
    pub created_on: Option<String>,
    pub updated_on: Option<String>,
    pub enabled_module_names: Option<Vec<String>>,
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
    pub project: ProjectToCreate,
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
