// Namespace: Composer\Test\EventDispatcher

#[derive(Debug, Clone, Default)]
pub struct EventDispatcherTest {
}

impl EventDispatcherTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tearDown(&self) {
        todo!()
    }

    pub fn testListenerExceptionsAreCaught(&self) {
        todo!()
    }

    pub fn testDispatcherCanExecuteSingleCommandLineScript(&self, command: String) {
        todo!()
    }

    pub fn testDispatcherPassDevModeToAutoloadGeneratorForScriptEvents(&self, devMode: bool) {
        todo!()
    }

    pub fn provideDevModes() -> Vec<serde_json::Value> {
        todo!()
    }

    fn getGeneratorMockForDevModePassingTest(&self) {
        todo!()
    }

    fn getRepositoryManagerMockForDevModePassingTest(&self) {
        todo!()
    }

    pub fn testDispatcherRemoveListener(&self) {
        todo!()
    }

    pub fn testDispatcherCanExecuteCliAndPhpInSameEventScriptStack(&self) {
        todo!()
    }

    pub fn testDispatcherCanPutEnv(&self) {
        todo!()
    }

    pub fn testDispatcherAppendsDirBinOnPathForEveryListener(&self) {
        todo!()
    }

    pub fn testDispatcherSupportForAdditionalArgs(&self) {
        todo!()
    }

    pub fn createsVendorBinFolderChecksEnvDoesNotContainsBin() {
        todo!()
    }

    pub fn createsVendorBinFolderChecksEnvContainsBin() {
        todo!()
    }

    pub fn getTestEnv() {
        todo!()
    }

    pub fn testDispatcherCanExecuteComposerScriptGroups(&self) {
        todo!()
    }

    pub fn testRecursionInScriptsNames(&self) {
        todo!()
    }

    pub fn testDispatcherDetectInfiniteRecursion(&self) {
        todo!()
    }

    fn getDispatcherStubForListenersTest(&self, listeners: Vec<serde_json::Value>, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>) {
        todo!()
    }

    pub fn provideValidCommands() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testDispatcherOutputsCommand(&self) {
        todo!()
    }

    pub fn testDispatcherOutputsErrorOnFailedCommand(&self) {
        todo!()
    }

    pub fn testDispatcherInstallerEvents(&self) {
        todo!()
    }

    pub fn testDispatcherDoesntReturnSkippedScripts(&self) {
        todo!()
    }

    pub fn call() {
        todo!()
    }

    pub fn someMethod() -> bool {
        todo!()
    }

    pub fn someMethod2() -> bool {
        todo!()
    }

    fn createComposerInstance(&self) -> crate::composer::composer::Composer {
        todo!()
    }

}

