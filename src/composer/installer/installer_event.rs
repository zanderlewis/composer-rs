// Namespace: Composer\Installer

#[derive(Debug, Clone, Default)]
pub struct InstallerEvent {
}

impl InstallerEvent {
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

    pub fn isExecutingOperations(&self) -> bool {
        todo!()
    }

    pub fn getTransaction(&self) -> Option<crate::composer::dependency_resolver::transaction::Transaction> {
        todo!()
    }

}

