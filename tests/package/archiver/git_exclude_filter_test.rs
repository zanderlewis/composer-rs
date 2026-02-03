// Namespace: Composer\Test\Package\Archiver

#[derive(Debug, Clone, Default)]
pub struct GitExcludeFilterTest {
}

impl GitExcludeFilterTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testPatternEscape(&self, ignore: String, expected: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn providePatterns() -> Vec<serde_json::Value> {
        todo!()
    }

}

