// Namespace: Composer\Command

// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct SelfUpdateCommand {
}

impl SelfUpdateCommand {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn configure(&self) {
        todo!()
    }

    pub(crate) fn execute(&self, input: serde_json::Value, output: serde_json::Value) -> i64 {
        todo!()
    }

    pub(crate) fn fetchKeys(&self, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>, config: crate::composer::config::Config) {
        todo!()
    }

    pub(crate) fn rollback(&self, output: serde_json::Value, rollbackDir: String, localFilename: String) -> i64 {
        todo!()
    }

    pub(crate) fn setLocalPhar(&self, localFilename: String, newFilename: String, backupTarget: Option<String>) -> bool {
        todo!()
    }

    pub(crate) fn cleanBackups(&self, rollbackDir: String, except: Option<String>) {
        todo!()
    }

    pub(crate) fn getLastBackupVersion(&self, rollbackDir: String) -> Option<String> {
        todo!()
    }

    pub(crate) fn getOldInstallationFinder(&self, rollbackDir: String) -> serde_json::Value {
        todo!()
    }

    pub(crate) fn validatePhar(&self, pharFile: String, error: Option<String>) -> bool {
        todo!()
    }

    pub(crate) fn isWindowsNonAdminUser(&self) -> bool {
        todo!()
    }

    pub(crate) fn tryAsWindowsAdmin(&self, localFilename: String, newFilename: String) -> bool {
        todo!()
    }

}

