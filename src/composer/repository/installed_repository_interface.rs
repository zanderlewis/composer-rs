// Namespace: Composer\Repository

pub trait InstalledRepositoryInterface {
    fn getDevMode(&self);
    fn isFresh(&self);
}

