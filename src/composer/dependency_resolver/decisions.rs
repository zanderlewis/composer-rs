// Namespace: Composer\DependencyResolver

// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct Decisions {
}

impl Decisions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn decide(&self, literal: i64, level: i64, why: crate::composer::dependency_resolver::rule::Rule) {
        todo!()
    }

    pub fn satisfy(&self, literal: i64) -> bool {
        todo!()
    }

    pub fn conflict(&self, literal: i64) -> bool {
        todo!()
    }

    pub fn decided(&self, literalOrPackageId: i64) -> bool {
        todo!()
    }

    pub fn undecided(&self, literalOrPackageId: i64) -> bool {
        todo!()
    }

    pub fn decidedInstall(&self, literalOrPackageId: i64) -> bool {
        todo!()
    }

    pub fn decisionLevel(&self, literalOrPackageId: i64) -> i64 {
        todo!()
    }

    pub fn decisionRule(&self, literalOrPackageId: i64) -> crate::composer::dependency_resolver::rule::Rule {
        todo!()
    }

    pub fn atOffset(&self, queueOffset: i64) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn validOffset(&self, queueOffset: i64) -> bool {
        todo!()
    }

    pub fn lastReason(&self) -> crate::composer::dependency_resolver::rule::Rule {
        todo!()
    }

    pub fn lastLiteral(&self) -> i64 {
        todo!()
    }

    pub fn reset(&self) {
        todo!()
    }

    pub fn resetToOffset(&self, offset: i64) {
        todo!()
    }

    pub fn revertLast(&self) {
        todo!()
    }

    pub fn count(&self) -> i64 {
        todo!()
    }

    pub fn rewind(&self) {
        todo!()
    }

    pub fn current(&self) {
        todo!()
    }

    pub fn key(&self) -> Option<i64> {
        todo!()
    }

    pub fn next(&self) {
        todo!()
    }

    pub fn valid(&self) -> bool {
        todo!()
    }

    pub fn isEmpty(&self) -> bool {
        todo!()
    }

    pub(crate) fn addDecision(&self, literal: i64, level: i64) {
        todo!()
    }

    pub fn toString(&self, pool: Option<crate::composer::dependency_resolver::pool::Pool>) -> String {
        todo!()
    }

    pub fn __toString(&self) -> String {
        todo!()
    }

}

