use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IdName {
    id: Option<i32>,
    name: Option<String>,
}
