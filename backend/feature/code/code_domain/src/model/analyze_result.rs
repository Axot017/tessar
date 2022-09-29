use super::lint::Lint;

#[derive(PartialEq, Eq, Debug)]
pub struct AnalyzeResult {
    pub success: bool,
    pub lints: Vec<Lint>,
}
