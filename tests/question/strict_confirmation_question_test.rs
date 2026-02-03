// Namespace: Composer\Test\Question

#[derive(Debug, Clone, Default)]
pub struct StrictConfirmationQuestionTest {
}

impl StrictConfirmationQuestionTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getAskConfirmationBadData() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testAskConfirmationBadAnswer(&self, answer: String) {
        todo!()
    }

    pub fn testAskConfirmation(&self, question: String, expected: bool, r#default: bool) {
        todo!()
    }

    pub fn getAskConfirmationData() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testAskConfirmationWithCustomTrueAndFalseAnswer(&self) {
        todo!()
    }

    pub(crate) fn getInputStream(&self, input: String) {
        todo!()
    }

    pub(crate) fn createOutputInterface(&self) -> serde_json::Value {
        todo!()
    }

    pub(crate) fn createInput(&self, entry: String) -> Vec<serde_json::Value> {
        todo!()
    }

}

