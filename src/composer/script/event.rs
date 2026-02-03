// Namespace: Composer\Script

#[derive(Debug, Clone, Default)]
pub struct Event {
}

impl Event {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getComposer(&self) -> crate::composer::composer::Composer {
        todo!()
    }

    pub fn getIO(&self) -> Box<dyn crate::composer::i_o::i_o_interface::IOInterface> {
        todo!()
    }

    pub fn isDevMode(&self) -> bool {
        todo!()
    }

    pub fn getOriginatingEvent(&self) -> Option<serde_json::Value> {
        todo!()
    }

    pub fn setOriginatingEvent(&self, event: serde_json::Value) -> Self {
        todo!()
    }

    fn calculateOriginatingEvent(&self, event: serde_json::Value) -> serde_json::Value {
        todo!()
    }

}

