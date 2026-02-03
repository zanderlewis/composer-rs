// Namespace: Composer\Package

#[derive(Debug, Clone, Default)]
pub struct AliasPackage {
}

impl AliasPackage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getAliasOf(&self) {
        todo!()
    }

    pub fn getVersion(&self) -> String {
        todo!()
    }

    pub fn getStability(&self) -> String {
        todo!()
    }

    pub fn getPrettyVersion(&self) -> String {
        todo!()
    }

    pub fn isDev(&self) -> bool {
        todo!()
    }

    pub fn getRequires(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getConflicts(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getProvides(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getReplaces(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getDevRequires(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn setRootPackageAlias(&self, value: bool) {
        todo!()
    }

    pub fn isRootPackageAlias(&self) -> bool {
        todo!()
    }

    pub(crate) fn replaceSelfVersionDependencies(&self, links: Vec<serde_json::Value>, linkType: serde_json::Value) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn hasSelfVersionRequires(&self) -> bool {
        todo!()
    }

    pub fn __toString(&self) -> String {
        todo!()
    }

    pub fn getType(&self) -> String {
        todo!()
    }

    pub fn getTargetDir(&self) -> Option<String> {
        todo!()
    }

    pub fn getExtra(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn setInstallationSource(&self, r#type: Option<String>) {
        todo!()
    }

    pub fn getInstallationSource(&self) -> Option<String> {
        todo!()
    }

    pub fn getSourceType(&self) -> Option<String> {
        todo!()
    }

    pub fn getSourceUrl(&self) -> Option<String> {
        todo!()
    }

    pub fn getSourceUrls(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getSourceReference(&self) -> Option<String> {
        todo!()
    }

    pub fn setSourceReference(&self, reference: Option<String>) {
        todo!()
    }

    pub fn setSourceMirrors(&self, mirrors: Option<Vec<serde_json::Value>>) {
        todo!()
    }

    pub fn getSourceMirrors(&self) -> Option<Vec<serde_json::Value>> {
        todo!()
    }

    pub fn getDistType(&self) -> Option<String> {
        todo!()
    }

    pub fn getDistUrl(&self) -> Option<String> {
        todo!()
    }

    pub fn getDistUrls(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getDistReference(&self) -> Option<String> {
        todo!()
    }

    pub fn setDistReference(&self, reference: Option<String>) {
        todo!()
    }

    pub fn getDistSha1Checksum(&self) -> Option<String> {
        todo!()
    }

    pub fn setTransportOptions(&self, options: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn getTransportOptions(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn setDistMirrors(&self, mirrors: Option<Vec<serde_json::Value>>) {
        todo!()
    }

    pub fn getDistMirrors(&self) -> Option<Vec<serde_json::Value>> {
        todo!()
    }

    pub fn getAutoload(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getDevAutoload(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getIncludePaths(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getPhpExt(&self) -> Option<Vec<serde_json::Value>> {
        todo!()
    }

    pub fn getReleaseDate(&self) -> Option<serde_json::Value> {
        todo!()
    }

    pub fn getBinaries(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getSuggests(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getNotificationUrl(&self) -> Option<String> {
        todo!()
    }

    pub fn isDefaultBranch(&self) -> bool {
        todo!()
    }

    pub fn setDistUrl(&self, url: Option<String>) {
        todo!()
    }

    pub fn setDistType(&self, r#type: Option<String>) {
        todo!()
    }

    pub fn setSourceDistReferences(&self, reference: String) {
        todo!()
    }

}

