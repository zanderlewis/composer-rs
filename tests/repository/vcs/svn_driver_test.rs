// Namespace: Composer\Test\Repository\Vcs

#[derive(Debug, Clone, Default)]
pub struct SvnDriverTest {
}

impl SvnDriverTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setUp(&self) {
        todo!()
    }

    pub(crate) fn tearDown(&self) {
        todo!()
    }

    pub fn testWrongCredentialsInUrl(&self) {
        todo!()
    }

    pub fn supportProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testSupport(&self, url: String, assertion: bool) {
        todo!()
    }

}

