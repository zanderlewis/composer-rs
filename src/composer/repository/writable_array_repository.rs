// Namespace: Composer\Repository

#[derive(Debug, Clone, Default)]
pub struct WritableArrayRepository {
}

impl WritableArrayRepository {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getDevMode(&self) {
        todo!()
    }

    pub fn setDevPackageNames(&self, devPackageNames: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn getDevPackageNames(&self) {
        todo!()
    }

    pub fn write(&self, devMode: bool, installationManager: crate::composer::installer::installation_manager::InstallationManager) {
        todo!()
    }

    pub fn reload(&self) {
        todo!()
    }

}

