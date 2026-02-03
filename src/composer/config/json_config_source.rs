// Namespace: Composer\Config

#[derive(Debug, Clone, Default)]
pub struct JsonConfigSource {
}

impl JsonConfigSource {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getName(&self) -> String {
        todo!()
    }

    pub fn addRepository(&self, name: String, config: serde_json::Value, append: bool) {
        todo!()
    }

    pub fn insertRepository(&self, name: String, config: serde_json::Value, referenceName: String, offset: i64) {
        todo!()
    }

    pub fn setRepositoryUrl(&self, name: String, url: String) {
        todo!()
    }

    pub fn removeRepository(&self, name: String) {
        todo!()
    }

    pub fn addConfigSetting(&self, name: String, value: serde_json::Value) {
        todo!()
    }

    pub fn removeConfigSetting(&self, name: String) {
        todo!()
    }

    pub fn addProperty(&self, name: String, value: serde_json::Value) {
        todo!()
    }

    pub fn removeProperty(&self, name: String) {
        todo!()
    }

    pub fn addLink(&self, r#type: String, name: String, value: String) {
        todo!()
    }

    pub fn removeLink(&self, r#type: String, name: String) {
        todo!()
    }

    fn manipulateJson(&self, method: String, fallback: Box<dyn Fn()>, args: serde_json::Value) {
        todo!()
    }

    fn arrayUnshiftRef(&self, array: Vec<serde_json::Value>, value: serde_json::Value) -> i64 {
        todo!()
    }

}

