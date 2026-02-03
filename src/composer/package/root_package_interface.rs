// Namespace: Composer\Package

pub trait RootPackageInterface {
    fn getAliases(&self) -> Vec<serde_json::Value>;
    fn getMinimumStability(&self) -> String;
    fn getStabilityFlags(&self) -> Vec<serde_json::Value>;
    fn getReferences(&self) -> Vec<serde_json::Value>;
    fn getPreferStable(&self) -> bool;
    fn getConfig(&self) -> Vec<serde_json::Value>;
    fn setRequires(&self, requires: Vec<serde_json::Value>);
    fn setDevRequires(&self, devRequires: Vec<serde_json::Value>);
    fn setConflicts(&self, conflicts: Vec<serde_json::Value>);
    fn setProvides(&self, provides: Vec<serde_json::Value>);
    fn setReplaces(&self, replaces: Vec<serde_json::Value>);
    fn setAutoload(&self, autoload: Vec<serde_json::Value>);
    fn setDevAutoload(&self, devAutoload: Vec<serde_json::Value>);
    fn setStabilityFlags(&self, stabilityFlags: Vec<serde_json::Value>);
    fn setMinimumStability(&self, minimumStability: String);
    fn setPreferStable(&self, preferStable: bool);
    fn setConfig(&self, config: Vec<serde_json::Value>);
    fn setReferences(&self, references: Vec<serde_json::Value>);
    fn setAliases(&self, aliases: Vec<serde_json::Value>);
    fn setSuggests(&self, suggests: Vec<serde_json::Value>);
    fn setExtra(&self, extra: Vec<serde_json::Value>);
}

