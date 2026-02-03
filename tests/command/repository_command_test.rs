// Namespace: Composer\Test\Command

#[derive(Debug, Clone, Default)]
pub struct RepositoryCommandTest {
}

impl RepositoryCommandTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testListWithNoRepositories(&self) {
        todo!()
    }

    pub fn testListWithRepositoriesAsList(&self) {
        todo!()
    }

    pub fn testListWithRepositoriesAsAssoc(&self) {
        todo!()
    }

    pub fn testAddRepositoryWithTypeAndUrl(&self) {
        todo!()
    }

    pub fn testAddRepositoryWithJson(&self) {
        todo!()
    }

    pub fn testRemoveRepository(&self) {
        todo!()
    }

    pub fn testSetAndGetUrlInRepositoryAssoc(&self, repositories: Vec<serde_json::Value>, name: String, index: String, newUrl: String) {
        todo!()
    }

    pub fn provideTestSetAndGetUrlInRepositoryAssoc() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testSetAndGetUrlInRepositoryList(&self, repositories: Vec<serde_json::Value>, name: String, index: i64, newUrl: String) {
        todo!()
    }

    pub fn provideTestSetAndGetUrlInRepositoryList() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testDisableAndEnablePackagist(&self) {
        todo!()
    }

    pub fn testInvalidArgCombinationThrows(&self) {
        todo!()
    }

    pub fn testPrependRepositoryByNameListToAssoc(&self) {
        todo!()
    }

    pub fn testAppendRepositoryByNameListToAssoc(&self) {
        todo!()
    }

    pub fn testPrependRepositoryAssocWithPackagistDisabled(&self) {
        todo!()
    }

    pub fn testAppendRepositoryAssocWithPackagistDisabled(&self) {
        todo!()
    }

    pub fn testAddBeforeAndAfterByName(&self) {
        todo!()
    }

    pub fn testAddSameNameReplacesExisting(&self) {
        todo!()
    }

}

