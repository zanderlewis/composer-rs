// Namespace: Composer\Config

pub trait ConfigSourceInterface {
    fn addRepository(&self, name: String, config: serde_json::Value, append: bool);
    fn insertRepository(&self, name: String, config: serde_json::Value, referenceName: String, offset: i64);
    fn setRepositoryUrl(&self, name: String, url: String);
    fn removeRepository(&self, name: String);
    fn addConfigSetting(&self, name: String, value: serde_json::Value);
    fn removeConfigSetting(&self, name: String);
    fn addProperty(&self, name: String, value: serde_json::Value);
    fn removeProperty(&self, name: String);
    fn addLink(&self, r#type: String, name: String, value: String);
    fn removeLink(&self, r#type: String, name: String);
    fn getName(&self) -> String;
}

