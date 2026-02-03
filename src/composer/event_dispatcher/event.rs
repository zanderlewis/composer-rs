// Namespace: Composer\EventDispatcher

#[derive(Debug, Clone, Default)]
pub struct Event {
}

impl Event {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getName(&self) -> String {
        todo!()
    }

    pub fn getArguments(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getFlags(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn isPropagationStopped(&self) -> bool {
        todo!()
    }

    pub fn stopPropagation(&self) {
        todo!()
    }

}

