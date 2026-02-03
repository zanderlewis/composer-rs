// Namespace: Composer\Package

pub trait CompletePackageInterface {
    fn getScripts(&self) -> Vec<serde_json::Value>;
    fn setScripts(&self, scripts: Vec<serde_json::Value>);
    fn getRepositories(&self) -> Vec<serde_json::Value>;
    fn setRepositories(&self, repositories: Vec<serde_json::Value>);
    fn getLicense(&self) -> Vec<serde_json::Value>;
    fn setLicense(&self, license: Vec<serde_json::Value>);
    fn getKeywords(&self) -> Vec<serde_json::Value>;
    fn setKeywords(&self, keywords: Vec<serde_json::Value>);
    fn getDescription(&self) -> Option<String>;
    fn setDescription(&self, description: String);
    fn getHomepage(&self) -> Option<String>;
    fn setHomepage(&self, homepage: String);
    fn getAuthors(&self) -> Vec<serde_json::Value>;
    fn setAuthors(&self, authors: Vec<serde_json::Value>);
    fn getSupport(&self) -> Vec<serde_json::Value>;
    fn setSupport(&self, support: Vec<serde_json::Value>);
    fn getFunding(&self) -> Vec<serde_json::Value>;
    fn setFunding(&self, funding: Vec<serde_json::Value>);
    fn isAbandoned(&self) -> bool;
    fn getReplacementPackage(&self) -> Option<String>;
    fn setAbandoned(&self, abandoned: serde_json::Value);
    fn getArchiveName(&self) -> Option<String>;
    fn setArchiveName(&self, name: String);
    fn getArchiveExcludes(&self) -> Vec<serde_json::Value>;
    fn setArchiveExcludes(&self, excludes: Vec<serde_json::Value>);
}

