// Namespace: Composer\Installer

#[derive(Debug, Clone, Default)]
pub struct PackageEvent {
}

impl PackageEvent {
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

    pub fn getLocalRepo(&self) -> Box<dyn crate::composer::repository::repository_interface::RepositoryInterface> {
        todo!()
    }

    pub fn getOperations(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getOperation(&self) -> Box<dyn crate::composer::dependency_resolver::operation::operation_interface::OperationInterface> {
        todo!()
    }

}

