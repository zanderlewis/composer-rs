// Namespace: Composer\Package\Loader

#[derive(Debug, Clone, Default)]
pub struct JsonLoader {
}

impl JsonLoader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(&self, json: serde_json::Value) -> crate::composer::package::base_package::BasePackage {
        todo!()
    }

}

