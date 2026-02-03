// Namespace: Composer\DependencyResolver

// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct SolverProblemsException {
}

impl SolverProblemsException {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getPrettyString(&self, repositorySet: crate::composer::repository::repository_set::RepositorySet, request: crate::composer::dependency_resolver::request::Request, pool: crate::composer::dependency_resolver::pool::Pool, isVerbose: bool, isDevExtraction: bool) -> String {
        todo!()
    }

    pub fn getProblems(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    fn createExtensionHint(&self, missingExtensions: Vec<serde_json::Value>) -> String {
        todo!()
    }

    fn getExtensionProblems(&self, reasonSets: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

}

