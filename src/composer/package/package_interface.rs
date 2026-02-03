// Namespace: Composer\Package

pub trait PackageInterface {
    fn getName(&self) -> String;
    fn getPrettyName(&self) -> String;
    fn getNames(&self, provides: bool) -> Vec<serde_json::Value>;
    fn setId(&self, id: i64);
    fn getId(&self) -> i64;
    fn isDev(&self) -> bool;
    fn getType(&self) -> String;
    fn getTargetDir(&self) -> Option<String>;
    fn getExtra(&self) -> Vec<serde_json::Value>;
    fn setInstallationSource(&self, r#type: Option<String>);
    fn getInstallationSource(&self) -> Option<String>;
    fn getSourceType(&self) -> Option<String>;
    fn getSourceUrl(&self) -> Option<String>;
    fn getSourceUrls(&self) -> Vec<serde_json::Value>;
    fn getSourceReference(&self) -> Option<String>;
    fn getSourceMirrors(&self) -> Option<Vec<serde_json::Value>>;
    fn setSourceMirrors(&self, mirrors: Option<Vec<serde_json::Value>>);
    fn getDistType(&self) -> Option<String>;
    fn getDistUrl(&self) -> Option<String>;
    fn getDistUrls(&self) -> Vec<serde_json::Value>;
    fn getDistReference(&self) -> Option<String>;
    fn getDistSha1Checksum(&self) -> Option<String>;
    fn getDistMirrors(&self) -> Option<Vec<serde_json::Value>>;
    fn setDistMirrors(&self, mirrors: Option<Vec<serde_json::Value>>);
    fn getVersion(&self) -> String;
    fn getPrettyVersion(&self) -> String;
    fn getFullPrettyVersion(&self, truncate: bool, displayMode: i64) -> String;
    fn getReleaseDate(&self) -> Option<serde_json::Value>;
    fn getStability(&self) -> String;
    fn getRequires(&self) -> Vec<serde_json::Value>;
    fn getConflicts(&self) -> Vec<serde_json::Value>;
    fn getProvides(&self) -> Vec<serde_json::Value>;
    fn getReplaces(&self) -> Vec<serde_json::Value>;
    fn getDevRequires(&self) -> Vec<serde_json::Value>;
    fn getSuggests(&self) -> Vec<serde_json::Value>;
    fn getAutoload(&self) -> Vec<serde_json::Value>;
    fn getDevAutoload(&self) -> Vec<serde_json::Value>;
    fn getIncludePaths(&self) -> Vec<serde_json::Value>;
    fn getPhpExt(&self) -> Option<Vec<serde_json::Value>>;
    fn setRepository(&self, repository: Box<dyn crate::composer::repository::repository_interface::RepositoryInterface>);
    fn getRepository(&self) -> Option<Box<dyn crate::composer::repository::repository_interface::RepositoryInterface>>;
    fn getBinaries(&self) -> Vec<serde_json::Value>;
    fn getUniqueName(&self) -> String;
    fn getNotificationUrl(&self) -> Option<String>;
    fn __toString(&self) -> String;
    fn getPrettyString(&self) -> String;
    fn isDefaultBranch(&self) -> bool;
    fn getTransportOptions(&self) -> Vec<serde_json::Value>;
    fn setTransportOptions(&self, options: Vec<serde_json::Value>);
    fn setSourceReference(&self, reference: Option<String>);
    fn setDistUrl(&self, url: Option<String>);
    fn setDistType(&self, r#type: Option<String>);
    fn setDistReference(&self, reference: Option<String>);
    fn setSourceDistReferences(&self, reference: String);
}

