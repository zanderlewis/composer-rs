// Namespace: Composer\Util

// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct ConfigValidator {
}

impl ConfigValidator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn validate(&self, file: String, arrayLoaderValidationFlags: i64, flags: i64) -> Vec<serde_json::Value> {
        todo!()
    }

}

