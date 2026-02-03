// Namespace: Composer\Test\Mock

#[derive(Debug, Clone, Default)]
pub struct FactoryMock {
}

impl FactoryMock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn createConfig(io: Option<Box<dyn crate::composer::i_o::i_o_interface::IOInterface>>, cwd: Option<String>) -> crate::composer::config::Config {
        todo!()
    }

    pub(crate) fn loadRootPackage(&self, rm: crate::composer::repository::repository_manager::RepositoryManager, config: crate::composer::config::Config, parser: crate::composer::package::version::version_parser::VersionParser, guesser: crate::composer::package::version::version_guesser::VersionGuesser, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>) -> crate::composer::package::loader::root_package_loader::RootPackageLoader {
        todo!()
    }

    pub(crate) fn addLocalRepository(&self, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>, rm: crate::composer::repository::repository_manager::RepositoryManager, vendorDir: String, rootPackage: Box<dyn crate::composer::package::root_package_interface::RootPackageInterface>, process: Option<crate::composer::util::process_executor::ProcessExecutor>) {
        todo!()
    }

    pub fn createInstallationManager(&self, r#loop: Option<crate::composer::util::loop_::Loop>, io: Option<Box<dyn crate::composer::i_o::i_o_interface::IOInterface>>, dispatcher: Option<crate::composer::event_dispatcher::event_dispatcher::EventDispatcher>) -> crate::composer::installer::installation_manager::InstallationManager {
        todo!()
    }

    pub(crate) fn createDefaultInstallers(&self, im: crate::composer::installer::installation_manager::InstallationManager, composer: crate::composer::partial_composer::PartialComposer, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>, process: Option<crate::composer::util::process_executor::ProcessExecutor>) {
        todo!()
    }

    pub(crate) fn purgePackages(&self, repo: Box<dyn crate::composer::repository::installed_repository_interface::InstalledRepositoryInterface>, im: crate::composer::installer::installation_manager::InstallationManager) {
        todo!()
    }

}

