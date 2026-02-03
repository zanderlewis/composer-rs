// Namespace: Composer\Test\DependencyResolver

#[derive(Debug, Clone, Default)]
pub struct PoolTest {
}

impl PoolTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testPool(&self) {
        todo!()
    }

    pub fn testWhatProvidesPackageWithConstraint(&self) {
        todo!()
    }

    pub fn testPackageById(&self) {
        todo!()
    }

    pub fn testWhatProvidesWhenPackageCannotBeFound(&self) {
        todo!()
    }

    pub(crate) fn createPool(&self, packages: Option<Vec<serde_json::Value>>) -> crate::composer::dependency_resolver::pool::Pool {
        todo!()
    }

}

