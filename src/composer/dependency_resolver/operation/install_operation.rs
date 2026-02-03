// Namespace: Composer\DependencyResolver\Operation

// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct InstallOperation {
}

impl InstallOperation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getPackage(&self) -> Box<dyn crate::composer::package::package_interface::PackageInterface> {
        todo!()
    }

    pub fn show(&self, lock: serde_json::Value) -> String {
        todo!()
    }

    pub fn format(package: Box<dyn crate::composer::package::package_interface::PackageInterface>, lock: bool) -> String {
        todo!()
    }

}

