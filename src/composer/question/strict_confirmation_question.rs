// Namespace: Composer\Question

#[derive(Debug, Clone, Default)]
pub struct StrictConfirmationQuestion {
}

impl StrictConfirmationQuestion {
    pub fn new() -> Self {
        Self::default()
    }

    fn getDefaultNormalizer(&self) -> Box<dyn Fn()> {
        todo!()
    }

    fn getDefaultValidator(&self) -> Box<dyn Fn()> {
        todo!()
    }

}

