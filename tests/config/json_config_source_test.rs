// Namespace: Composer\Test\Config

#[derive(Debug, Clone, Default)]
pub struct JsonConfigSourceTest {
}

impl JsonConfigSourceTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn fixturePath(name: String) -> String {
        todo!()
    }

    pub(crate) fn setUp(&self) {
        todo!()
    }

    pub(crate) fn tearDown(&self) {
        todo!()
    }

    pub fn testAddRepository(&self) {
        todo!()
    }

    pub fn testAddRepositoryAsList(&self) {
        todo!()
    }

    pub fn testAddRepositoryWithOptions(&self) {
        todo!()
    }

    pub fn testRemoveRepository(&self) {
        todo!()
    }

    pub fn testAddPackagistRepositoryWithFalseValue(&self) {
        todo!()
    }

    pub fn testRemovePackagist(&self) {
        todo!()
    }

    pub fn testAddLink(&self, sourceFile: String, r#type: String, name: String, value: String, compareAgainst: String) {
        todo!()
    }

    pub fn testRemoveLink(&self, sourceFile: String, r#type: String, name: String, compareAgainst: String) {
        todo!()
    }

    pub(crate) fn addLinkDataArguments(r#type: String, name: String, value: String, fixtureBasename: String, before: String) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn provideAddLinkData() -> Vec<serde_json::Value> {
        todo!()
    }

    pub(crate) fn removeLinkDataArguments(r#type: String, name: String, fixtureBasename: String, after: Option<String>) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn provideRemoveLinkData() -> Vec<serde_json::Value> {
        todo!()
    }

}

