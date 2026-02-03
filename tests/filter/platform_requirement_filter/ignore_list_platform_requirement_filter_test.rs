// Namespace: Composer\Test\Filter\PlatformRequirementFilter

#[derive(Debug, Clone, Default)]
pub struct IgnoreListPlatformRequirementFilterTest {
}

impl IgnoreListPlatformRequirementFilterTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testIsIgnored(&self, reqList: Vec<serde_json::Value>, req: String, expectIgnored: bool) {
        todo!()
    }

    pub fn dataIsIgnored() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testIsUpperBoundIgnored(&self, reqList: Vec<serde_json::Value>, req: String, expectIgnored: bool) {
        todo!()
    }

    pub fn dataIsUpperBoundIgnored() -> Vec<serde_json::Value> {
        todo!()
    }

}

