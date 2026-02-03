// Namespace: Composer\Test\DependencyResolver

#[derive(Debug, Clone, Default)]
pub struct DefaultPolicyTest {
}

impl DefaultPolicyTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setUp(&self) {
        todo!()
    }

    pub(crate) fn tearDown(&self) {
        todo!()
    }

    pub fn testSelectSingle(&self) {
        todo!()
    }

    pub fn testSelectNewest(&self) {
        todo!()
    }

    pub fn testSelectNewestPicksLatest(&self) {
        todo!()
    }

    pub fn testSelectNewestPicksLatestStableWithPreferStable(&self) {
        todo!()
    }

    pub fn testSelectLowestWithPreferDevOverPrerelease(&self, stability: String) {
        todo!()
    }

    pub fn testSelectLowestPrefersPrereleaseOverDev(&self, stability: String) {
        todo!()
    }

    pub fn testSelectLowestWithPreferStableStillPrefersStable(&self) {
        todo!()
    }

    pub fn testSelectNewestWithDevPicksNonDev(&self) {
        todo!()
    }

    pub fn testSelectNewestWithPreferredVersionPicksPreferredVersionIfAvailable(&self) {
        todo!()
    }

    pub fn testSelectNewestWithPreferredVersionPicksNewestOtherwise(&self) {
        todo!()
    }

    pub fn testSelectNewestWithPreferredVersionPicksLowestIfPreferLowest(&self) {
        todo!()
    }

    pub fn testRepositoryOrderingAffectsPriority(&self) {
        todo!()
    }

    pub fn testSelectLocalReposFirst(&self) {
        todo!()
    }

    pub fn testSelectAllProviders(&self) {
        todo!()
    }

    pub fn testPreferNonReplacingFromSameRepo(&self) {
        todo!()
    }

    pub fn testPreferReplacingPackageFromSameVendor(&self) {
        todo!()
    }

    pub fn testSelectLowest(&self) {
        todo!()
    }

}

