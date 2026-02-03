// Namespace: Composer\DependencyResolver

#[derive(Debug, Clone, Default)]
pub struct RuleWatchNode {
}

impl RuleWatchNode {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn watch2OnHighest(&self, decisions: crate::composer::dependency_resolver::decisions::Decisions) {
        todo!()
    }

    pub fn getRule(&self) -> crate::composer::dependency_resolver::rule::Rule {
        todo!()
    }

    pub fn getOtherWatch(&self, literal: i64) -> i64 {
        todo!()
    }

    pub fn moveWatch(&self, from: i64, to: i64) {
        todo!()
    }

}

