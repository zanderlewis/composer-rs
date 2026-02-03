// Namespace: Composer\Test

#[derive(Debug, Clone, Default)]
pub struct DocumentationTest {
}

impl DocumentationTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testCommand(&self, command: crate::tests::composer::test::plugin::fixtures::plugin_v8::installer::command_provider::Command) {
        todo!()
    }

    fn getCommandName(&self, command: crate::tests::composer::test::plugin::fixtures::plugin_v8::installer::command_provider::Command) -> String {
        todo!()
    }

    pub fn provideCommandCases() -> serde_json::Value {
        todo!()
    }

}

