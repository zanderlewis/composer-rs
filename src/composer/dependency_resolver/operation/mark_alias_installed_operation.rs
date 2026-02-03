// Namespace: Composer\DependencyResolver\Operation

// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct MarkAliasInstalledOperation {
}

impl MarkAliasInstalledOperation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getPackage(&self) -> crate::composer::package::alias_package::AliasPackage {
        todo!()
    }

    pub fn show(&self, lock: serde_json::Value) -> String {
        todo!()
    }

}

