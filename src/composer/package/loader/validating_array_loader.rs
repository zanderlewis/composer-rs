// Namespace: Composer\Package\Loader

// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct ValidatingArrayLoader {
}

impl ValidatingArrayLoader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(&self, config: Vec<serde_json::Value>, class: String) -> crate::composer::package::base_package::BasePackage {
        todo!()
    }

    pub fn getWarnings(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getErrors(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn hasPackageNamingError(name: String, isLink: bool) -> Option<String> {
        todo!()
    }

    fn validateRegex(&self, property: String, regex: String, mandatory: bool) -> bool {
        todo!()
    }

    fn validateString(&self, property: String, mandatory: bool) -> bool {
        todo!()
    }

    fn validateArray(&self, property: String, mandatory: bool) -> bool {
        todo!()
    }

    fn validateFlatArray(&self, property: String, regex: Option<String>, mandatory: bool) -> bool {
        todo!()
    }

    fn validateUrl(&self, property: String, mandatory: bool) -> bool {
        todo!()
    }

    fn filterUrl(&self, value: serde_json::Value, schemes: Vec<serde_json::Value>) -> bool {
        todo!()
    }

}

