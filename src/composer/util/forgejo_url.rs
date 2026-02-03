// Namespace: Composer\Util

// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct ForgejoUrl {
}

impl ForgejoUrl {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create(repoUrl: String) -> Self {
        todo!()
    }

    pub fn tryFrom(repoUrl: Option<String>) -> Option<Self> {
        todo!()
    }

    pub fn generateSshUrl(&self) -> String {
        todo!()
    }

}

