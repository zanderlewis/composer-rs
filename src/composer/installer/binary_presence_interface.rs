// Namespace: Composer\Installer

pub trait BinaryPresenceInterface {
    fn ensureBinariesPresence(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>);
}

