// Namespace: Composer\Repository\Vcs

#[derive(Debug, Clone, Default)]
pub struct VcsDriver {
}

impl VcsDriver {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn shouldCache(&self, identifier: String) -> bool {
        todo!()
    }

    pub fn getComposerInformation(&self, identifier: String) -> Option<Vec<serde_json::Value>> {
        todo!()
    }

    pub(crate) fn getBaseComposerInformation(&self, identifier: String) -> Option<Vec<serde_json::Value>> {
        todo!()
    }

    pub fn hasComposerFile(&self, identifier: String) -> bool {
        todo!()
    }

    pub(crate) fn getScheme(&self) -> String {
        todo!()
    }

    pub(crate) fn getContents(&self, url: String) -> crate::composer::util::http::response::Response {
        todo!()
    }

    pub fn cleanup(&self) {
        todo!()
    }

}

