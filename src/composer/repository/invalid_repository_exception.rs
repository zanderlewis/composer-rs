// Namespace: Composer\Repository

#[derive(Debug, Clone, Default)]
pub struct InvalidRepositoryException {
}

impl InvalidRepositoryException {
    pub fn new() -> Self {
        Self::default()
    }

}

