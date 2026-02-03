// Namespace: Composer\Util

#[derive(Debug, Clone, Default)]
pub struct GitLab {
}

impl GitLab {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn authorizeOAuth(&self, originUrl: String) -> bool {
        todo!()
    }

    pub fn authorizeOAuthInteractively(&self, scheme: String, originUrl: String, message: Option<String>) -> bool {
        todo!()
    }

    pub fn authorizeOAuthRefresh(&self, scheme: String, originUrl: String) -> bool {
        todo!()
    }

    fn createToken(&self, scheme: String, originUrl: String) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn isOAuthExpired(&self, originUrl: String) -> bool {
        todo!()
    }

    fn refreshToken(&self, scheme: String, originUrl: String) -> Vec<serde_json::Value> {
        todo!()
    }

}

