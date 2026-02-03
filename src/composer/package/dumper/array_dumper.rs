// Namespace: Composer\Package\Dumper

#[derive(Debug, Clone, Default)]
pub struct ArrayDumper {
}

impl ArrayDumper {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dump(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) -> Vec<serde_json::Value> {
        todo!()
    }

    fn dumpValues(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, keys: Vec<serde_json::Value>, data: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

}

