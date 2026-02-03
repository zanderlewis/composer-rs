// Namespace: Composer\Repository\Vcs

pub trait VcsDriverInterface {
    fn initialize(&self);
    fn getComposerInformation(&self, identifier: String) -> Option<Vec<serde_json::Value>>;
    fn getFileContent(&self, file: String, identifier: String) -> Option<String>;
    fn getChangeDate(&self, identifier: String) -> Option<serde_json::Value>;
    fn getRootIdentifier(&self) -> String;
    fn getBranches(&self) -> Vec<serde_json::Value>;
    fn getTags(&self) -> Vec<serde_json::Value>;
    fn getDist(&self, identifier: String) -> Option<Vec<serde_json::Value>>;
    fn getSource(&self, identifier: String) -> Vec<serde_json::Value>;
    fn getUrl(&self) -> String;
    fn hasComposerFile(&self, identifier: String) -> bool;
    fn cleanup(&self);
    fn supports(&self, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>, config: crate::composer::config::Config, url: String, deep: bool) -> bool;
}

