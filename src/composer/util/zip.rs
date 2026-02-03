// Namespace: Composer\Util

#[derive(Debug, Clone, Default)]
pub struct Zip {
}

impl Zip {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getComposerJson(pathToZip: String) -> Option<String> {
        todo!()
    }

    fn locateFile(zip: serde_json::Value, filename: String) -> i64 {
        todo!()
    }

}

