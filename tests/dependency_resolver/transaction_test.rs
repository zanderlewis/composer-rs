// Namespace: Composer\Test\DependencyResolver

#[derive(Debug, Clone, Default)]
pub struct TransactionTest {
}

impl TransactionTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setUp(&self) {
        todo!()
    }

    pub fn testTransactionGenerationAndSorting(&self) {
        todo!()
    }

    pub(crate) fn checkTransactionOperations(&self, transaction: crate::composer::dependency_resolver::transaction::Transaction, expected: Vec<serde_json::Value>) {
        todo!()
    }

}

