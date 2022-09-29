use super::lint_level::LintLevel;

#[derive(PartialEq, Eq, Debug)]
pub struct Lint {
    pub file: String,
    pub message: String,
    pub column: u32,
    pub row: u32,
    pub lint_level: LintLevel,
}
