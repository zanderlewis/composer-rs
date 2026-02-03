// Namespace: Composer\Util

#[derive(Debug, Clone, Default)]
pub struct Forgejo {
}

impl Forgejo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn authorizeOAuthInteractively(&self, originUrl: String, message: Option<String>) -> bool {
        todo!()
    }

}

