// Namespace: Composer\Plugin

#[derive(Debug, Clone, Default)]
pub struct PostFileDownloadEvent {
}

impl PostFileDownloadEvent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getFileName(&self) -> Option<String> {
        todo!()
    }

    pub fn getChecksum(&self) -> Option<String> {
        todo!()
    }

    pub fn getUrl(&self) -> String {
        todo!()
    }

    pub fn getContext(&self) {
        todo!()
    }

    pub fn getPackage(&self) -> Option<Box<dyn crate::composer::package::package_interface::PackageInterface>> {
        todo!()
    }

    pub fn getType(&self) -> String {
        todo!()
    }

}

