// Namespace: Composer\Util

#[derive(Debug, Clone, Default)]
pub struct Silencer {
}

impl Silencer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn suppress(mask: Option<i64>) -> i64 {
        todo!()
    }

    pub fn restore() {
        todo!()
    }

    pub fn call(callable: Box<dyn Fn()>, parameters: serde_json::Value) {
        todo!()
    }

}

