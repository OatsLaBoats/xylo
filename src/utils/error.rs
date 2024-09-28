use super::SourceInfo;

#[derive(Clone, Debug)]
pub struct Error {
    pub si: SourceInfo,
    pub message: String,
}

impl Error {
    pub fn new(message: String, si: SourceInfo) -> Self {
        Self {
            message,
            si,
        }
    }

    pub fn to_string(&self, file_name: &str) -> String {
        format!("ERROR::{file_name}({}:{}) {}", self.si.line, self.si.column, &self.message)
    }
}
