// Namespace: Composer\Util

#[derive(Debug, Clone, Default)]
pub struct ErrorHandler {
}

impl ErrorHandler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle(level: i64, message: String, file: String, line: i64) -> bool {
        todo!()
    }

    pub fn register(io: Option<Box<dyn crate::composer::i_o::i_o_interface::IOInterface>>) {
        todo!()
    }

    fn outputWarning(message: String, outputEvenWithoutIO: bool) {
        todo!()
    }

}

