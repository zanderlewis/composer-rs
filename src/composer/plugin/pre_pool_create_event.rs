// Namespace: Composer\Plugin

#[derive(Debug, Clone, Default)]
pub struct PrePoolCreateEvent {
}

impl PrePoolCreateEvent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getRepositories(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getRequest(&self) -> crate::composer::dependency_resolver::request::Request {
        todo!()
    }

    pub fn getAcceptableStabilities(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getStabilityFlags(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getRootAliases(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getRootReferences(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getPackages(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getUnacceptableFixedPackages(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn setPackages(&self, packages: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn setUnacceptableFixedPackages(&self, packages: Vec<serde_json::Value>) {
        todo!()
    }

}

