// Namespace: Composer\DependencyResolver

// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct RuleSet {
}

impl RuleSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&self, rule: crate::composer::dependency_resolver::rule::Rule, r#type: serde_json::Value) {
        todo!()
    }

    pub fn count(&self) -> i64 {
        todo!()
    }

    pub fn ruleById(&self, id: i64) -> crate::composer::dependency_resolver::rule::Rule {
        todo!()
    }

    pub fn getRules(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getIterator(&self) -> crate::composer::dependency_resolver::rule_set_iterator::RuleSetIterator {
        todo!()
    }

    pub fn getIteratorFor(&self, types: serde_json::Value) -> crate::composer::dependency_resolver::rule_set_iterator::RuleSetIterator {
        todo!()
    }

    pub fn getIteratorWithout(&self, types: serde_json::Value) -> crate::composer::dependency_resolver::rule_set_iterator::RuleSetIterator {
        todo!()
    }

    pub fn getTypes(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getPrettyString(&self, repositorySet: Option<crate::composer::repository::repository_set::RepositorySet>, request: Option<crate::composer::dependency_resolver::request::Request>, pool: Option<crate::composer::dependency_resolver::pool::Pool>, isVerbose: bool) -> String {
        todo!()
    }

    pub fn __toString(&self) -> String {
        todo!()
    }

}

