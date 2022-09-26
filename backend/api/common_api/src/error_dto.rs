use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct ErrorDto {
    pub message: String,
    pub code: String,
    pub args: Option<HashMap<String, String>>,
}
