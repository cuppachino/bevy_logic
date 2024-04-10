use bevy::prelude::*;
use derive_more::From;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::components::*;
    pub use super::gates::{ AndGate };
}

pub mod gates;
pub mod components {
    use super::*;

    /// An output node.
    #[derive(Component, Clone, Copy, Debug)]
    pub struct Source {
        pub signal: Signal,
    }

    /// An input node.
    #[derive(Component, Clone, Copy, Debug)]
    pub struct Sink {
        pub signal: Signal,
    }

    /// A connection between two nodes.
    #[derive(Component, Clone, Copy, Debug)]
    pub struct Wire {
        pub source: Entity,
        pub sink: Entity,
    }
}

/// State storage for logic simulation.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, From)]
pub enum Signal {
    Analog(f32),
    Digital(bool),
    Undefined,
}

impl Signal {
    pub const OFF: Signal = Signal::Digital(false);
    pub const ON: Signal = Signal::Digital(true);

    /// Returns true if the signal is true or greater or equal to 1.0.
    pub fn is_true(&self) -> bool {
        match self {
            Signal::Digital(true) => true,
            Signal::Analog(value) => *value >= 1.0,
            _ => false,
        }
    }
}

impl std::ops::Add for Signal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Signal::Undefined, _) | (_, Signal::Undefined) => Signal::Undefined,
            (Signal::Analog(lhs), Signal::Analog(rhs)) => Signal::Analog(lhs + rhs),
            (Signal::Analog(a), Signal::Digital(d)) | (Signal::Digital(d), Signal::Analog(a)) =>
                Signal::Analog(a + (if d { 1.0 } else { 0.0 })),

            (Signal::Digital(lhs), Signal::Digital(rhs)) => Signal::Digital(lhs || rhs),
        }
    }
}

impl std::ops::Add<f32> for Signal {
    type Output = Self;
    fn add(self, rhs: f32) -> Self::Output {
        match self {
            Signal::Analog(value) => Signal::Analog(value + rhs),
            Signal::Digital(true) => Signal::Analog(1.0 + rhs),
            Signal::Digital(false) => Signal::Analog(rhs),
            Signal::Undefined => Signal::Undefined,
        }
    }
}
