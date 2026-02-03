// Namespace: Composer\DependencyResolver\Operation

pub trait OperationInterface {
    fn getOperationType(&self);
    fn show(&self, lock: bool);
    fn __toString(&self);
}

