// Namespace: Composer

#[derive(Debug, Clone, Default)]
pub struct PartialComposer {
}

impl PartialComposer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setPackage(&self, package: Box<dyn crate::composer::package::root_package_interface::RootPackageInterface>) {
        todo!()
    }

    pub fn getPackage(&self) -> Box<dyn crate::composer::package::root_package_interface::RootPackageInterface> {
        todo!()
    }

    pub fn setConfig(&self, config: crate::composer::config::Config) {
        todo!()
    }

    pub fn getConfig(&self) -> crate::composer::config::Config {
        todo!()
    }

    pub fn setLoop(&self, r#loop: crate::composer::util::loop_::Loop) {
        todo!()
    }

    pub fn getLoop(&self) -> crate::composer::util::loop_::Loop {
        todo!()
    }

    pub fn setRepositoryManager(&self, manager: crate::composer::repository::repository_manager::RepositoryManager) {
        todo!()
    }

    pub fn getRepositoryManager(&self) -> crate::composer::repository::repository_manager::RepositoryManager {
        todo!()
    }

    pub fn setInstallationManager(&self, manager: crate::composer::installer::installation_manager::InstallationManager) {
        todo!()
    }

    pub fn getInstallationManager(&self) -> crate::composer::installer::installation_manager::InstallationManager {
        todo!()
    }

    pub fn setEventDispatcher(&self, eventDispatcher: crate::composer::event_dispatcher::event_dispatcher::EventDispatcher) {
        todo!()
    }

    pub fn getEventDispatcher(&self) -> crate::composer::event_dispatcher::event_dispatcher::EventDispatcher {
        todo!()
    }

    pub fn isGlobal(&self) -> bool {
        todo!()
    }

    pub fn setGlobal(&self) {
        todo!()
    }

}

