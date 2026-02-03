// Namespace: Composer\Command

#[derive(Debug, Clone, Default)]
pub struct FundCommand {
}

impl FundCommand {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn configure(&self) {
        todo!()
    }

    pub(crate) fn execute(&self, input: serde_json::Value, output: serde_json::Value) -> i64 {
        todo!()
    }

    fn insertFundingData(&self, fundings: Vec<serde_json::Value>, package: Box<dyn crate::composer::package::complete_package_interface::CompletePackageInterface>) -> Vec<serde_json::Value> {
        todo!()
    }

}

