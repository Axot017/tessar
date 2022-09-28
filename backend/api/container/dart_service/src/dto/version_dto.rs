use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct VersionDto {
    pub message: String,
}
