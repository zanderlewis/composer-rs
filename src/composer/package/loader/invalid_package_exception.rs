// Namespace: Composer\Package\Loader

#[derive(Debug, Clone, Default)]
pub struct InvalidPackageException {
}

impl InvalidPackageException {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getData(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getErrors(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getWarnings(&self) -> Vec<serde_json::Value> {
        todo!()
    }

}

