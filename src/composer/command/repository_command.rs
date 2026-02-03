// Namespace: Composer\Command

#[derive(Debug, Clone, Default)]
pub struct RepositoryCommand {
}

impl RepositoryCommand {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn configure(&self) {
        todo!()
    }

    pub(crate) fn execute(&self, input: serde_json::Value, output: serde_json::Value) -> i64 {
        todo!()
    }

    fn listRepositories(&self, repos: Vec<serde_json::Value>) {
        todo!()
    }

    fn suggestTypeForAdd(&self) -> serde_json::Value {
        todo!()
    }

    fn suggestRepoNames(&self) -> serde_json::Value {
        todo!()
    }

}

