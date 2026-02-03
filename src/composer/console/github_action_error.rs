// Namespace: Composer\Console

#[derive(Debug, Clone, Default)]
pub struct GithubActionError {
}

impl GithubActionError {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn emit(&self, message: String, file: Option<String>, line: Option<i64>) {
        todo!()
    }

    fn escapeData(&self, data: String) -> String {
        todo!()
    }

    fn escapeProperty(&self, property: String) -> String {
        todo!()
    }

}

