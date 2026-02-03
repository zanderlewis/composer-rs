// Namespace: Composer\Repository

pub trait VersionCacheInterface {
    fn getVersionPackage(&self, version: String, identifier: String);
}

