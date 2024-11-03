use bevy::prelude::*;

/// State storage for logic simulation.
#[derive(Component, Clone, Copy, Debug, PartialEq, PartialOrd, Reflect)]
pub enum Signal {
    Analog(f32),
    Digital(bool),
    Undefined,
}

impl std::fmt::Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Signal::Analog(value) => write!(f, "{:.2}", value),
            Signal::Digital(value) => write!(f, "{}", value),
            Signal::Undefined => write!(f, "Undefined"),
        }
    }
}

impl From<f32> for Signal {
    fn from(value: f32) -> Self {
        Signal::Analog(value)
    }
}

impl From<bool> for Signal {
    fn from(value: bool) -> Self {
        Signal::Digital(value)
    }
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

    /// Returns `true` if the signal is `Digital(true)` or `Analog(_normal_float_)`.
    ///
    /// # Example
    ///
    /// ```
    /// assert!(Signal::ON.is_truthy());
    /// assert!(Signal::Analog(0.1).is_truthy());
    /// assert!(Signal::Analog(-0.1).is_truthy());
    /// ```
    pub fn is_truthy(&self) -> bool {
        match self {
            Signal::Digital(true) => true,
            Signal::Analog(value) => value.is_normal(),
            _ => false,
        }
    }

    /// Returns true if the signal is `Digital(false) or Analog(_non_normal_float_)`.
    ///
    /// # Example
    ///
    /// ```
    /// assert!(Signal::OFF.is_falsy());
    /// assert!(Signal::Analog(0.0).is_falsy());
    /// assert!(Signal::Analog(f32::NAN).is_falsy());
    /// assert!(Signal::Undefined.is_falsy());
    /// ```
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

    /// Replace self with a new signal if the new signal is not equal.
    ///
    /// This is useful for preventing unnecessary changes from triggering
    /// bevy's change detection.
    pub fn replace(&mut self, new: Self) {
        if new != *self {
            *self = new;
        }
    }

    /// Returns true if the signal is "truthy", [`Analog`] and negative.
    ///
    /// [`Analog`]: Signal::Analog
    pub fn is_sign_negative(&self) -> bool {
        match self {
            Self::Analog(value) => value.is_sign_negative(),
            _ => false,
        }
    }

    /// Returns true if the signal is "truthy", [`Analog`] and positive
    ///
    /// [`Analog`]: Signal::Analog
    pub fn is_sign_positive(&self) -> bool {
        match self {
            Self::Analog(value) => value.is_sign_positive(),
            _ => false,
        }
    }

    /// Returns `true` if the signal is [`Analog`].
    ///
    /// [`Analog`]: Signal::Analog
    #[must_use]
    pub fn is_analog(&self) -> bool {
        matches!(self, Self::Analog(..))
    }

    /// Returns `true` if the signal is [`Digital`].
    ///
    /// [`Digital`]: Signal::Digital
    #[must_use]
    pub fn is_digital(&self) -> bool {
        matches!(self, Self::Digital(..))
    }

    /// Returns `true` if the signal is [`Undefined`].
    ///
    /// [`Undefined`]: Signal::Undefined
    #[must_use]
    pub fn is_undefined(&self) -> bool {
        matches!(self, Self::Undefined)
    }

    /// Compare two signals and return the signal with a greater
    /// absolute value.
    ///
    /// This is useful when negative signals should hold the same weight as positive signals.
    pub fn max_abs(self, other: Signal) -> Signal {
        match (self, other) {
            // Analog cmp Analog
            (Signal::Analog(a), Signal::Analog(b)) => {
                if a.abs() >= b.abs() { Signal::Analog(a) } else { Signal::Analog(b) }
            }
            // Analog cmp Digital
            (Signal::Analog(a), Signal::OFF) | (Signal::OFF, Signal::Analog(a)) => {
                if a.is_normal() { Signal::Analog(a) } else { Signal::OFF }
            }
            (Signal::Analog(a), Signal::ON) | (Signal::ON, Signal::Analog(a)) => {
                if a.abs() >= 1.0 { Signal::Analog(a) } else { Signal::ON }
            }
            // Digital cmp Digital
            (Signal::OFF, Signal::OFF) => Signal::OFF,
            (Signal::ON, Signal::ON) | (Signal::ON, Signal::OFF) | (Signal::OFF, Signal::ON) => {
                Signal::ON
            }
            // Undefined
            (Signal::Undefined, v) | (v, Signal::Undefined) => v,
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
