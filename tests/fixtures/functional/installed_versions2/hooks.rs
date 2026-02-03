#[derive(Debug, Clone, Default)]
pub struct Hooks {
}

impl Hooks {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn preUpdate(event: crate::composer::script::event::Event) {
        todo!()
    }

    pub fn postUpdate(event: crate::composer::script::event::Event) {
        todo!()
    }

}

