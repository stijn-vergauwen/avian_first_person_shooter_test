use std::fmt::Debug;

#[derive(Default, Debug, Clone, Copy)]
pub struct PdControllerValues<T: Copy + Default + Debug> {
    pub position: T,
    pub velocity: T,
    pub acceleration: T,
}
