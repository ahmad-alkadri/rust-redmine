use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IdName {
    id: Option<i32>,
    name: Option<String>,
}
