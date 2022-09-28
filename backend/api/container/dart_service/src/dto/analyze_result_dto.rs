use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct AnalyzeResultDto {
    pub success: bool,
    pub lints: Vec<Lint>,
}

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct Lint {
    pub level: String,
    pub message: String,
    pub file: String,
    pub column: u32,
    pub row: u32,
}
