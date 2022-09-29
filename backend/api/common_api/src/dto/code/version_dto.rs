use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct VersionDto {
    pub message: String,
}

impl From<String> for VersionDto {
    fn from(s: String) -> Self {
        VersionDto { message: s }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_string() {
        assert_eq!(
            VersionDto::from("x.y.z".to_owned()),
            VersionDto {
                message: "x.y.z".to_owned()
            }
        )
    }
}
