// Namespace: Composer\Util

#[derive(Debug, Clone, Default)]
pub struct Hg {
}

impl Hg {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn runCommand(&self, commandCallable: Box<dyn Fn()>, url: String, cwd: Option<String>) {
        todo!()
    }

    fn throwException(&self, message: serde_json::Value, url: String) {
        todo!()
    }

    pub fn getVersion(process: crate::composer::util::process_executor::ProcessExecutor) -> Option<String> {
        todo!()
    }

}

