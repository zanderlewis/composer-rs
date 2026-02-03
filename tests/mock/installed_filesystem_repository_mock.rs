// Namespace: Composer\Test\Mock

#[derive(Debug, Clone, Default)]
pub struct InstalledFilesystemRepositoryMock {
}

impl InstalledFilesystemRepositoryMock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reload(&self) {
        todo!()
    }

    pub fn write(&self, devMode: serde_json::Value, installationManager: crate::composer::installer::installation_manager::InstallationManager) {
        todo!()
    }

}

