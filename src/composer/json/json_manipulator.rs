// Namespace: Composer\Json

// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct JsonManipulator {
}

impl JsonManipulator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getContents(&self) -> String {
        todo!()
    }

    pub fn addLink(&self, r#type: String, package: String, constraint: String, sortPackages: bool) -> bool {
        todo!()
    }

    fn sortPackages(&self, packages: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn addRepository(&self, name: String, config: serde_json::Value, append: bool) -> bool {
        todo!()
    }

    fn doConvertRepositoriesFromAssocToList(&self) -> bool {
        todo!()
    }

    pub fn setRepositoryUrl(&self, name: String, url: String) -> bool {
        todo!()
    }

    pub fn insertRepository(&self, name: String, config: serde_json::Value, referenceName: String, offset: i64) -> bool {
        todo!()
    }

    pub fn removeRepository(&self, name: String) -> bool {
        todo!()
    }

    fn doRemoveRepository(&self, name: String) -> bool {
        todo!()
    }

    pub fn addConfigSetting(&self, name: String, value: serde_json::Value) -> bool {
        todo!()
    }

    pub fn removeConfigSetting(&self, name: String) -> bool {
        todo!()
    }

    pub fn addProperty(&self, name: String, value: serde_json::Value) -> bool {
        todo!()
    }

    pub fn removeProperty(&self, name: String) -> bool {
        todo!()
    }

    pub fn addSubNode(&self, mainNode: String, name: String, value: serde_json::Value, append: bool) -> bool {
        todo!()
    }

    pub fn removeSubNode(&self, mainNode: String, name: String) -> bool {
        todo!()
    }

    pub fn addListItem(&self, mainNode: String, value: serde_json::Value, append: bool) -> bool {
        todo!()
    }

    pub fn insertListItem(&self, mainNode: String, value: serde_json::Value, index: i64) -> bool {
        todo!()
    }

    pub fn removeListItem(&self, mainNode: String, nodeIndex: i64) -> bool {
        todo!()
    }

    pub fn addMainKey(&self, key: String, content: serde_json::Value) -> bool {
        todo!()
    }

    pub fn removeMainKey(&self, key: String) -> bool {
        todo!()
    }

    pub fn changeEmptyMainKeyFromAssocToList(&self, key: String) -> bool {
        todo!()
    }

    pub fn removeMainKeyIfEmpty(&self, key: String) -> bool {
        todo!()
    }

    pub fn format(&self, data: serde_json::Value, depth: i64, wasObject: bool) -> String {
        todo!()
    }

    pub(crate) fn detectIndenting(&self) {
        todo!()
    }

}

