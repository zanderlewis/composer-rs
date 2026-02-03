// Namespace: Composer\Util

#[derive(Debug, Clone, Default)]
pub struct ForgejoRepositoryData {
}

impl ForgejoRepositoryData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fromRemoteData(data: Vec<serde_json::Value>) -> Self {
        todo!()
    }

}

