// Namespace: Composer\EventDispatcher

#[derive(Debug, Clone, Default)]
pub struct EventDispatcher {
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setRunScripts(&self, runScripts: bool) -> Self {
        todo!()
    }

    pub fn dispatch(&self, eventName: Option<String>, event: Option<crate::composer::script::event::Event>) -> i64 {
        todo!()
    }

    pub fn dispatchScript(&self, eventName: String, devMode: bool, additionalArgs: Vec<serde_json::Value>, flags: Vec<serde_json::Value>) -> i64 {
        todo!()
    }

    pub fn dispatchPackageEvent(&self, eventName: String, devMode: bool, localRepo: Box<dyn crate::composer::repository::repository_interface::RepositoryInterface>, operations: Vec<serde_json::Value>, operation: Box<dyn crate::composer::dependency_resolver::operation::operation_interface::OperationInterface>) -> i64 {
        todo!()
    }

    pub fn dispatchInstallerEvent(&self, eventName: String, devMode: bool, executeOperations: bool, transaction: crate::composer::dependency_resolver::transaction::Transaction) -> i64 {
        todo!()
    }

    pub(crate) fn doDispatch(&self, event: crate::composer::script::event::Event) {
        todo!()
    }

    pub(crate) fn executeTty(&self, exec: String) -> i64 {
        todo!()
    }

    pub(crate) fn getPhpExecCommand(&self) -> String {
        todo!()
    }

    pub(crate) fn executeEventPhpScript(&self, className: String, methodName: String, event: crate::composer::script::event::Event) {
        todo!()
    }

    fn eventNeedsToOutput(&self, event: crate::composer::script::event::Event) -> bool {
        todo!()
    }

    pub fn addListener(&self, eventName: String, listener: serde_json::Value, priority: i64) {
        todo!()
    }

    pub fn removeListener(&self, listener: serde_json::Value) {
        todo!()
    }

    pub fn addSubscriber(&self, subscriber: Box<dyn crate::composer::event_dispatcher::event_subscriber_interface::EventSubscriberInterface>) {
        todo!()
    }

    pub(crate) fn getListeners(&self, event: crate::composer::script::event::Event) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn hasEventListeners(&self, event: crate::composer::script::event::Event) -> bool {
        todo!()
    }

    pub(crate) fn getScriptListeners(&self, event: crate::composer::script::event::Event) -> Vec<serde_json::Value> {
        todo!()
    }

    pub(crate) fn isPhpScript(&self, callable: String) -> bool {
        todo!()
    }

    pub(crate) fn isCommandClass(&self, callable: String) -> bool {
        todo!()
    }

    pub(crate) fn isComposerScript(&self, callable: String) -> bool {
        todo!()
    }

    pub(crate) fn pushEvent(&self, event: crate::composer::script::event::Event) -> i64 {
        todo!()
    }

    pub(crate) fn popEvent(&self) -> Option<String> {
        todo!()
    }

    fn ensureBinDirIsInPath(&self) {
        todo!()
    }

    fn getCallbackIdentifier(&self, cb: serde_json::Value) -> String {
        todo!()
    }

    fn makeAutoloader(&self, event: crate::composer::script::event::Event, callable: serde_json::Value) {
        todo!()
    }

}

