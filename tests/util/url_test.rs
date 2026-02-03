// Namespace: Composer\Test\Util

#[derive(Debug, Clone, Default)]
pub struct UrlTest {
}

impl UrlTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testUpdateDistReference(&self, url: String, expectedUrl: String, conf: Vec<serde_json::Value>, r#ref: String) {
        todo!()
    }

    pub fn distRefsProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testSanitize(&self, expected: String, url: String) {
        todo!()
    }

    pub fn sanitizeProvider() -> Vec<serde_json::Value> {
        todo!()
    }

}

