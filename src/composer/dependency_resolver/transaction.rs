// Namespace: Composer\DependencyResolver

#[derive(Debug, Clone, Default)]
pub struct Transaction {
}

impl Transaction {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getOperations(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    fn setResultPackageMaps(&self, resultPackages: Vec<serde_json::Value>) {
        todo!()
    }

    pub(crate) fn calculateOperations(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub(crate) fn getRootPackages(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub(crate) fn getProvidersInResult(&self, link: crate::composer::package::link::Link) -> Vec<serde_json::Value> {
        todo!()
    }

    fn movePluginsToFront(&self, operations: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    fn moveUninstallsToFront(&self, operations: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

}

