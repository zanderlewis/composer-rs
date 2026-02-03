// Namespace: Composer\Util

#[derive(Debug, Clone, Default)]
pub struct AuthHelper {
}

impl AuthHelper {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn storeAuth(&self, origin: String, storeAuth: serde_json::Value) {
        todo!()
    }

    pub fn promptAuthIfNeeded(&self, url: String, origin: String, statusCode: i64, reason: Option<String>, headers: Vec<serde_json::Value>, retryCount: i64) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn addAuthenticationHeader(&self, headers: Vec<serde_json::Value>, origin: String, url: String) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn addAuthenticationOptions(&self, options: Vec<serde_json::Value>, origin: String, url: String) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn isPublicBitBucketDownload(&self, urlToBitBucketFile: String) -> bool {
        todo!()
    }

}

