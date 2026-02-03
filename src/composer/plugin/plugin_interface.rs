// Namespace: Composer\Plugin

pub trait PluginInterface {
    fn activate(&self, composer: crate::composer::composer::Composer, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>);
    fn deactivate(&self, composer: crate::composer::composer::Composer, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>);
    fn uninstall(&self, composer: crate::composer::composer::Composer, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>);
}

