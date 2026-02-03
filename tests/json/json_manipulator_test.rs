// Namespace: Composer\Test\Json

#[derive(Debug, Clone, Default)]
pub struct JsonManipulatorTest {
}

impl JsonManipulatorTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testAddLink(&self, json: String, r#type: String, package: String, constraint: String, expected: String) {
        todo!()
    }

    pub fn linkProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testAddLinkAndSortPackages(&self, json: String, r#type: String, package: String, constraint: String, sortPackages: bool, expected: String) {
        todo!()
    }

    pub fn providerAddLinkAndSortPackages() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testRemoveSubNode(&self, json: String, name: String, expected: bool, expectedContent: Option<String>) {
        todo!()
    }

    pub fn removeSubNodeProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testRemoveSubNodeFromRequire(&self) {
        todo!()
    }

    pub fn testRemoveSubNodePreservesObjectTypeWhenEmpty(&self) {
        todo!()
    }

    pub fn testRemoveSubNodePreservesObjectTypeWhenEmpty2(&self) {
        todo!()
    }

    pub fn testAddSubNodeInRequire(&self) {
        todo!()
    }

    pub fn testAddExtraWithPackage(&self) {
        todo!()
    }

    pub fn testAddConfigWithPackage(&self) {
        todo!()
    }

    pub fn testAddSuggestWithPackage(&self) {
        todo!()
    }

    pub fn testAddRepositoryCanInitializeEmptyRepositories(&self) {
        todo!()
    }

    pub fn testAddRepositoryCanInitializeFromScratch(&self) {
        todo!()
    }

    pub fn testAddRepositoryCanAppend(&self) {
        todo!()
    }

    pub fn testAddRepositoryCanPrepend(&self) {
        todo!()
    }

    pub fn addRepositoryProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testAddRepository(&self, from: String, to: String, name: String, config: serde_json::Value, append: bool) {
        todo!()
    }

    pub fn testAddRepositoryCanOverrideDeepRepos(&self) {
        todo!()
    }

    pub fn testSetUrlInRepository(&self, from: String, to: String, name: String, url: String) {
        todo!()
    }

    pub fn provideTestSetUrlInRepository() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testInsertRepositoryBeforeAndAfterByName(&self) {
        todo!()
    }

    pub fn testRemoveRepositoryRemovesFromAssocButDoesNotConvertsFromAssocToList(&self) {
        todo!()
    }

    pub fn testRemoveRepositoryRemovesFromList(&self) {
        todo!()
    }

    pub fn testAddRepositoryConvertsFromAssocToList(&self) {
        todo!()
    }

    pub fn testAddConfigSettingEscapes(&self) {
        todo!()
    }

    pub fn testAddConfigSettingWorksFromScratch(&self) {
        todo!()
    }

    pub fn testAddConfigSettingCanAdd(&self) {
        todo!()
    }

    pub fn testAddConfigSettingCanOverwrite(&self) {
        todo!()
    }

    pub fn testAddConfigSettingCanOverwriteNumbers(&self) {
        todo!()
    }

    pub fn testAddConfigSettingCanOverwriteArrays(&self) {
        todo!()
    }

    pub fn testAddConfigSettingCanAddSubKeyInEmptyConfig(&self) {
        todo!()
    }

    pub fn testAddConfigSettingCanAddSubKeyInEmptyVal(&self) {
        todo!()
    }

    pub fn testAddConfigSettingCanAddSubKeyInHash(&self) {
        todo!()
    }

    pub fn testAddRootSettingDoesNotBreakDots(&self) {
        todo!()
    }

    pub fn testRemoveConfigSettingCanRemoveSubKeyInHash(&self) {
        todo!()
    }

    pub fn testRemoveConfigSettingCanRemoveSubKeyInHashWithSiblings(&self) {
        todo!()
    }

    pub fn testAddMainKey(&self) {
        todo!()
    }

    pub fn testAddMainKeyWithContentHavingDollarSignFollowedByDigit(&self) {
        todo!()
    }

    pub fn testAddMainKeyWithContentHavingDollarSignFollowedByDigit2(&self) {
        todo!()
    }

    pub fn testUpdateMainKey(&self) {
        todo!()
    }

    pub fn testUpdateMainKey2(&self) {
        todo!()
    }

    pub fn testUpdateMainKey3(&self) {
        todo!()
    }

    pub fn testUpdateMainKeyWithContentHavingDollarSignFollowedByDigit(&self) {
        todo!()
    }

    pub fn testRemoveMainKey(&self) {
        todo!()
    }

    pub fn testRemoveMainKeyIfEmpty(&self) {
        todo!()
    }

    pub fn testRemoveMainKeyRemovesKeyWhereValueIsNull(&self) {
        todo!()
    }

    pub fn testIndentDetection(&self) {
        todo!()
    }

    pub fn testRemoveMainKeyAtEndOfFile(&self) {
        todo!()
    }

    pub fn testAddListItem(&self, from: String, to: String, mainNode: String, value: serde_json::Value, append: bool) {
        todo!()
    }

    pub fn addListItemProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testRemoveListItem(&self, from: String, to: String, mainNode: String, indexToRemove: i64) {
        todo!()
    }

    pub fn removeListItemProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testInsertListItem(&self, from: String, to: String, mainNode: String, value: Vec<serde_json::Value>, indexToInsertAt: i64) {
        todo!()
    }

    pub fn insertListItemProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testEscapedUnicodeDoesNotCauseBacktrackLimitErrorGithubIssue8131(&self) {
        todo!()
    }

    pub fn testLargeFileDoesNotCauseBacktrackLimitErrorGithubIssue9595(&self) {
        todo!()
    }

}

