// Namespace: Composer

#[derive(Debug, Clone, Default)]
pub struct Cache {
}

impl Cache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setReadOnly(&self, readOnly: bool) {
        todo!()
    }

    pub fn isReadOnly(&self) {
        todo!()
    }

    pub fn isUsable(path: String) {
        todo!()
    }

    pub fn isEnabled(&self) {
        todo!()
    }

    pub fn getRoot(&self) {
        todo!()
    }

    pub fn read(&self, file: String) {
        todo!()
    }

    pub fn write(&self, file: String, contents: String) {
        todo!()
    }

    pub fn copyFrom(&self, file: String, source: String) {
        todo!()
    }

    pub fn copyTo(&self, file: String, target: String) {
        todo!()
    }

    pub fn gcIsNecessary(&self) {
        todo!()
    }

    pub fn remove(&self, file: String) {
        todo!()
    }

    pub fn clear(&self) {
        todo!()
    }

    pub fn getAge(&self, file: String) {
        todo!()
    }

    pub fn gc(&self, ttl: i64, maxSize: i64) {
        todo!()
    }

    pub fn gcVcsCache(&self, ttl: i64) -> bool {
        todo!()
    }

    pub fn sha1(&self, file: String) {
        todo!()
    }

    pub fn sha256(&self, file: String) {
        todo!()
    }

    pub(crate) fn getFinder(&self) {
        todo!()
    }

}

