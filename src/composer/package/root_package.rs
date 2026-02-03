// Namespace: Composer\Package

// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct RootPackage {
}

impl RootPackage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setMinimumStability(&self, minimumStability: String) {
        todo!()
    }

    pub fn getMinimumStability(&self) -> String {
        todo!()
    }

    pub fn setStabilityFlags(&self, stabilityFlags: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn getStabilityFlags(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn setPreferStable(&self, preferStable: bool) {
        todo!()
    }

    pub fn getPreferStable(&self) -> bool {
        todo!()
    }

    pub fn setConfig(&self, config: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn getConfig(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn setReferences(&self, references: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn getReferences(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn setAliases(&self, aliases: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn getAliases(&self) -> Vec<serde_json::Value> {
        todo!()
    }

}

