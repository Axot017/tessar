use code_domain::model::analyze_result::AnalyzeResult;
use code_domain::model::lint::Lint;
use code_domain::model::lint_level::LintLevel;
use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct AnalyzeResultDto {
    pub success: bool,
    pub lints: Vec<LintDto>,
}

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct LintDto {
    pub level: LintLevelDto,
    pub message: String,
    pub file: String,
    pub column: u32,
    pub row: u32,
}

#[derive(Serialize, PartialEq, Eq, Debug)]
pub enum LintLevelDto {
    ERROR,
    WARNING,
    INFO,
}

impl From<LintLevel> for LintLevelDto {
    fn from(level: LintLevel) -> Self {
        match level {
            LintLevel::ERROR => LintLevelDto::ERROR,
            LintLevel::WARNING => LintLevelDto::WARNING,
            LintLevel::INFO => LintLevelDto::INFO,
        }
    }
}

impl From<Lint> for LintDto {
    fn from(lint: Lint) -> Self {
        Self {
            level: lint.lint_level.into(),
            message: lint.message,
            file: lint.file,
            column: lint.column,
            row: lint.row,
        }
    }
}

impl From<AnalyzeResult> for AnalyzeResultDto {
    fn from(result: AnalyzeResult) -> Self {
        AnalyzeResultDto {
            success: result.success,
            lints: result.lints.into_iter().map(LintDto::from).collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_analyze_result() {
        let model = AnalyzeResult {
            success: true,
            lints: vec![Lint {
                file: "some_file.dart".to_owned(),
                message: "file_content".to_owned(),
                column: 1,
                row: 2,
                lint_level: LintLevel::INFO,
            }],
        };
        let expected_dto = AnalyzeResultDto {
            success: true,
            lints: vec![LintDto {
                file: "some_file.dart".to_owned(),
                message: "file_content".to_owned(),
                column: 1,
                row: 2,
                level: LintLevelDto::INFO,
            }],
        };

        assert_eq!(AnalyzeResultDto::from(model), expected_dto)
    }
}
