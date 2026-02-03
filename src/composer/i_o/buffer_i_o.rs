// Namespace: Composer\IO

#[derive(Debug, Clone, Default)]
pub struct BufferIO {
}

impl BufferIO {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getOutput(&self) -> String {
        todo!()
    }

    pub fn setUserInputs(&self, inputs: Vec<serde_json::Value>) {
        todo!()
    }

    fn createStream(&self, inputs: Vec<serde_json::Value>) {
        todo!()
    }

}

