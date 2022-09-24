use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    BuildFailed,
    FailedToFetchFlutter,
    FailedToPrecacheFlutter,
    DartDoesNotExist,
    FailedToCreateDartProject,
    FailedToAddDartPackage(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

pub type DynError = Box<dyn std::error::Error>;
