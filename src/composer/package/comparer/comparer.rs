// Namespace: Composer\Package\Comparer

#[derive(Debug, Clone, Default)]
pub struct Comparer {
}

impl Comparer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setSource(&self, source: String) {
        todo!()
    }

    pub fn setUpdate(&self, update: String) {
        todo!()
    }

    pub fn getChanged(&self, explicated: bool) {
        todo!()
    }

    pub fn getChangedAsString(&self, toString: bool, explicated: bool) -> String {
        todo!()
    }

    pub fn doCompare(&self) {
        todo!()
    }

    fn doTree(&self, dir: String, array: Vec<serde_json::Value>) {
        todo!()
    }

}

