// Namespace: Composer\Package

// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct Link {
}

impl Link {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getDescription(&self) -> String {
        todo!()
    }

    pub fn getSource(&self) -> String {
        todo!()
    }

    pub fn getTarget(&self) -> String {
        todo!()
    }

    pub fn getConstraint(&self) -> serde_json::Value {
        todo!()
    }

    pub fn getPrettyConstraint(&self) -> String {
        todo!()
    }

    pub fn __toString(&self) -> String {
        todo!()
    }

    pub fn getPrettyString(&self, sourcePackage: Box<dyn crate::composer::package::package_interface::PackageInterface>) -> String {
        todo!()
    }

}

