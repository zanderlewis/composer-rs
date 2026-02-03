// Namespace: Composer\Util

#[derive(Debug, Clone, Default)]
pub struct Tar {
}

impl Tar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getComposerJson(pathToArchive: String) -> Option<String> {
        todo!()
    }

    fn extractComposerJsonFromFolder(phar: serde_json::Value) -> String {
        todo!()
    }

}

