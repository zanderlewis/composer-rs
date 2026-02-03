// Namespace: Installer

#[derive(Debug, Clone, Default)]
pub struct Plugin6 {
}

impl Plugin6 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn activate(&self, composer: crate::composer::composer::Composer, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>) {
        todo!()
    }

    pub fn deactivate(&self, composer: crate::composer::composer::Composer, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>) {
        todo!()
    }

    pub fn uninstall(&self, composer: crate::composer::composer::Composer, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>) {
        todo!()
    }

}

