use bevy::prelude::*;
use derive_more::{ From, Display };

/// State storage for logic simulation.
#[derive(Component, Clone, Copy, Debug, Display, PartialEq, PartialOrd, From, Reflect)]
pub enum Signal {
    Analog(f32),
    Digital(bool),
    Undefined,
}

impl Default for Signal {
    fn default() -> Self {
        Signal::Undefined
    }
}

impl Signal {
    pub const OFF: Signal = Signal::Digital(false);
    pub const ON: Signal = Signal::Digital(true);
    pub const NEG: Signal = Signal::Analog(-1.0);

    /// Returns true if the signal is true or greater or equal to 1.0.
    pub fn is_truthy(&self) -> bool {
        match self {
            Signal::Digital(true) => true,
            Signal::Analog(value) => value.is_normal(),
            _ => false,
        }
    }

    /// Returns true if the signal is false, less than 1.0, or undefined.
    pub fn is_falsy(&self) -> bool {
        match self {
            Signal::Digital(true) => false,
            Signal::Analog(value) => !value.is_normal(),
            _ => true,
        }
    }

    /// Set the signal to `Digital(false)`.
    pub fn turn_off(&mut self) {
        *self = Signal::OFF;
    }

    // Set the signal to `Digital(true)`.
    pub fn turn_on(&mut self) {
        *self = Signal::ON;
    }

    /// Replace self with a new signal.
    pub fn replace(&mut self, new: Self) {
        *self = new;
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

impl std::ops::Sub for Signal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Signal::Undefined, _) | (_, Signal::Undefined) => Signal::Undefined,
            (Signal::Analog(lhs), Signal::Analog(rhs)) => Signal::Analog(lhs - rhs),
            (Signal::Analog(a), Signal::Digital(true)) => Signal::Analog(a - 1.0),
            (Signal::Analog(a), Signal::Digital(false)) => Signal::Analog(a),
            (Signal::Digital(true), Signal::Analog(a)) => Signal::Analog(1.0 - a),
            (Signal::Digital(false), Signal::Analog(a)) => Signal::Analog(-a),
            (Signal::Digital(true), Signal::Digital(false)) => Signal::Digital(true),
            _ => Signal::Digital(false),
        }
    }
}

impl std::ops::Sub<f32> for Signal {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        match self {
            Signal::Analog(value) => Signal::Analog(value - rhs),
            Signal::Digital(true) => Signal::Analog(1.0 - rhs),
            Signal::Digital(false) => Signal::Analog(-rhs),
            Signal::Undefined => Signal::Undefined,
        }
    }
}

impl std::ops::Not for Signal {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Signal::Analog(value) => Signal::Analog(-value),
            Signal::Digital(value) => Signal::Digital(!value),
            Signal::Undefined => Signal::Undefined,
        }
    }
}

pub trait SignalExt {
    /// Replace all signals in `self` with `signal`.
    fn set_all(&mut self, signal: Signal);
}

impl SignalExt for Vec<Signal> {
    fn set_all(&mut self, signal: Signal) {
        self.iter_mut().for_each(|s| {
            *s = signal;
        });
    }
}

impl SignalExt for [Signal] {
    fn set_all(&mut self, signal: Signal) {
        self.iter_mut().for_each(|s| {
            *s = signal;
        });
    }
}
