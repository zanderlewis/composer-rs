// Namespace: Composer\Test\Repository

#[derive(Debug, Clone, Default)]
pub struct PathRepositoryTest {
}

impl PathRepositoryTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testLoadPackageFromFileSystemWithIncorrectPath(&self) {
        todo!()
    }

    pub fn testLoadPackageFromFileSystemWithVersion(&self) {
        todo!()
    }

    pub fn testLoadPackageFromFileSystemWithoutVersion(&self) {
        todo!()
    }

    pub fn testLoadPackageFromFileSystemWithWildcard(&self) {
        todo!()
    }

    pub fn testLoadPackageWithExplicitVersions(&self) {
        todo!()
    }

    pub fn testUrlRemainsRelative(&self) {
        todo!()
    }

    pub fn testReferenceNone(&self) {
        todo!()
    }

    pub fn testReferenceConfig(&self) {
        todo!()
    }

    fn createPathRepo(&self, options: Vec<serde_json::Value>) -> crate::composer::repository::path_repository::PathRepository {
        todo!()
    }

}

