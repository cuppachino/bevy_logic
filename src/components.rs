use bevy::prelude::*;
use derive_new::new;

pub mod prelude {
    pub use super::ComponentRef;
}

#[derive(new, Component, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ComponentRef<T> {
    pub entity: Entity,
    _marker: std::marker::PhantomData<T>,
}
