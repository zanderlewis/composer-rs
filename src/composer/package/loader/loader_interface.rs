// Namespace: Composer\Package\Loader

pub trait LoaderInterface {
    fn load(&self, config: Vec<serde_json::Value>, class: String) -> crate::composer::package::base_package::BasePackage;
}

