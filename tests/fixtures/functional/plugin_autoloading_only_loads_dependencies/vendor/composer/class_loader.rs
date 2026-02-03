// Namespace: Composer\Autoload

#[derive(Debug, Clone, Default)]
pub struct ClassLoader {
}

impl ClassLoader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getPrefixes(&self) {
        todo!()
    }

    pub fn getPrefixesPsr4(&self) {
        todo!()
    }

    pub fn getFallbackDirs(&self) {
        todo!()
    }

    pub fn getFallbackDirsPsr4(&self) {
        todo!()
    }

    pub fn getClassMap(&self) {
        todo!()
    }

    pub fn addClassMap(&self, classMap: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn add(&self, prefix: serde_json::Value, paths: serde_json::Value, prepend: serde_json::Value) {
        todo!()
    }

    pub fn addPsr4(&self, prefix: serde_json::Value, paths: serde_json::Value, prepend: serde_json::Value) {
        todo!()
    }

    pub fn set(&self, prefix: serde_json::Value, paths: serde_json::Value) {
        todo!()
    }

    pub fn setPsr4(&self, prefix: serde_json::Value, paths: serde_json::Value) {
        todo!()
    }

    pub fn setUseIncludePath(&self, useIncludePath: serde_json::Value) {
        todo!()
    }

    pub fn getUseIncludePath(&self) {
        todo!()
    }

    pub fn setClassMapAuthoritative(&self, classMapAuthoritative: serde_json::Value) {
        todo!()
    }

    pub fn isClassMapAuthoritative(&self) {
        todo!()
    }

    pub fn setApcuPrefix(&self, apcuPrefix: serde_json::Value) {
        todo!()
    }

    pub fn getApcuPrefix(&self) {
        todo!()
    }

    pub fn register(&self, prepend: serde_json::Value) {
        todo!()
    }

    pub fn unregister(&self) {
        todo!()
    }

    pub fn loadClass(&self, class: serde_json::Value) {
        todo!()
    }

    pub fn findFile(&self, class: serde_json::Value) {
        todo!()
    }

    pub fn getRegisteredLoaders() {
        todo!()
    }

    fn findFileWithExtension(&self, class: serde_json::Value, ext: serde_json::Value) {
        todo!()
    }

}

pub fn includeFile(file: serde_json::Value) {
    todo!()
}

