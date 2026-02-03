// Namespace: Composer\DependencyResolver

#[derive(Debug, Clone, Default)]
pub struct MultiConflictRule {
}

impl MultiConflictRule {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getLiterals(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getHash(&self) {
        todo!()
    }

    pub fn equals(&self, rule: crate::composer::dependency_resolver::rule::Rule) -> bool {
        todo!()
    }

    pub fn isAssertion(&self) -> bool {
        todo!()
    }

    pub fn disable(&self) {
        todo!()
    }

    pub fn __toString(&self) -> String {
        todo!()
    }

}

