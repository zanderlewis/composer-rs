// Namespace: Composer\SelfUpdate

// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct Versions {
}

impl Versions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getChannel(&self) -> String {
        todo!()
    }

    pub fn setChannel(&self, channel: String, io: Option<Box<dyn crate::composer::i_o::i_o_interface::IOInterface>>) {
        todo!()
    }

    pub fn getLatest(&self, channel: Option<String>) -> Vec<serde_json::Value> {
        todo!()
    }

    fn getVersionsData(&self) -> Vec<serde_json::Value> {
        todo!()
    }

}

