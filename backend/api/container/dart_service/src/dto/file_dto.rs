use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct FileDto {
    pub path: String,
    pub content: String,
}
