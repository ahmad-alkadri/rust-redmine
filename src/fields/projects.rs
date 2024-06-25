use crate::fields::issues::IdName;
use serde::{Deserialize, Serialize};

// type Project struct {
// 	Id                 int            `json:"id"`
// 	Parent             IdName         `json:"parent"`
// 	Name               string         `json:"name"`
// 	Identifier         string         `json:"identifier"`
// 	Description        string         `json:"description"`
// 	CreatedOn          string         `json:"created_on"`
// 	UpdatedOn          string         `json:"updated_on"`
// 	CustomFields       []*CustomField `json:"custom_fields,omitempty"`
// 	EnabledModuleNames []string       `json:"enabled_module_names,omitempty"`
// }

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
