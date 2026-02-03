// Namespace: Composer\Advisory

#[derive(Debug, Clone, Default)]
pub struct SecurityAdvisory {
}

impl SecurityAdvisory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn toIgnoredAdvisory(&self, ignoreReason: Option<String>) -> crate::composer::advisory::ignored_security_advisory::IgnoredSecurityAdvisory {
        todo!()
    }

    pub fn jsonSerialize(&self) {
        todo!()
    }

}

