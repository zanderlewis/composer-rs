// Namespace: Composer\Package\Archiver

#[derive(Debug, Clone, Default)]
pub struct ArchivableFilesFilter {
}

impl ArchivableFilesFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn accept(&self) -> bool {
        todo!()
    }

    pub fn addEmptyDir(&self, phar: serde_json::Value, sources: String) {
        todo!()
    }

}

