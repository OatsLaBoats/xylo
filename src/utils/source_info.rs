#[derive(Default, Debug, Clone, Copy)]
pub struct SourceInfo {
    pub line: i64,
    pub column: i64,
    pub index: usize,
}

impl SourceInfo {
    pub fn new(
        line: i64,
        column: i64,
        index: usize,
    ) -> Self {
        Self { line, column, index }
    }
}
