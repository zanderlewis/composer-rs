// Namespace: Composer\DependencyResolver\Operation

// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct UpdateOperation {
}

impl UpdateOperation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getInitialPackage(&self) -> Box<dyn crate::composer::package::package_interface::PackageInterface> {
        todo!()
    }

    pub fn getTargetPackage(&self) -> Box<dyn crate::composer::package::package_interface::PackageInterface> {
        todo!()
    }

    pub fn show(&self, lock: serde_json::Value) -> String {
        todo!()
    }

    pub fn format(initialPackage: Box<dyn crate::composer::package::package_interface::PackageInterface>, targetPackage: Box<dyn crate::composer::package::package_interface::PackageInterface>, lock: bool) -> String {
        todo!()
    }

}

