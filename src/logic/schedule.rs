use std::time::Duration;

use bevy::{ app::FixedMain, ecs::schedule::ScheduleLabel, prelude::* };

/// A plugin that initializes the [`LogicUpdate`] schedule for a given [`App`].
///
/// This works just like bevy's [`FixedUpdate`] schedule. The speed of the simulation
/// can be controlled by inserting a [`Time<LogicStep>`] resource.
///
/// See [`FixedMain`] for more information.
pub struct LogicSchedulePlugin;

impl Plugin for LogicSchedulePlugin {
    fn build(&self, app: &mut App) {
        app.init_schedule(LogicUpdate).add_systems(FixedMain, run_fixed_main_schedule);
    }
}

/// A label for the fixed logic update schedule.
///
/// The speed of the simulation can be controlled by inserting a [`Time<LogicStep>`] resource.
#[derive(ScheduleLabel, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct LogicUpdate;

pub fn run_fixed_main_schedule(world: &mut World) {
    let delta = world.resource::<Time<Virtual>>().delta();
    world.resource_mut::<Time<LogicStep>>().accumulate(delta);

    // Run the schedule until we run out of accumulated time
    let _ = world.try_schedule_scope(LogicUpdate, |world, schedule| {
        while world.resource_mut::<Time<LogicStep>>().expend() {
            *world.resource_mut::<Time>() = world.resource::<Time<LogicStep>>().as_generic();
            schedule.run(world);
        }
    });

    *world.resource_mut::<Time>() = world.resource::<Time<Virtual>>().as_generic();
}

/// A fixed timestep context for logic simulation.
#[derive(Clone, Copy, Debug, Default, Reflect)]
pub struct LogicStep {
    timestep: Duration,
    overstep: Duration,
}

impl FixedLogicStepExt for Time<LogicStep> {
    #[inline]
    fn context(&self) -> &LogicStep {
        self.context()
    }

    #[inline]
    fn context_mut(&mut self) -> &mut LogicStep {
        self.context_mut()
    }

    #[inline]
    fn advance_by(&mut self, delta: Duration) {
        self.advance_by(delta);
    }
}

/// Adapted from Bevy source code.
pub trait FixedLogicStepExt: Default {
    /// Corresponds to 64 Hz.
    const DEFAULT_TIMESTEP: Duration = Duration::from_micros(15625);

    /// Return new fixed time clock with given timestep as [`Duration`]
    ///
    /// # Panics
    ///
    /// Panics if `timestep` is zero.
    fn from_duration(timestep: Duration) -> Self {
        let mut ret = Self::default();
        ret.set_timestep(timestep);
        ret
    }

    /// Return new fixed time clock with given timestep seconds as `f64`
    ///
    /// # Panics
    ///
    /// Panics if `seconds` is zero, negative or not finite.
    fn from_seconds(seconds: f64) -> Self {
        let mut ret = Self::default();
        ret.set_timestep_seconds(seconds);
        ret
    }

    /// Return new fixed time clock with given timestep frequency in Hertz (1/seconds)
    ///
    /// # Panics
    ///
    /// Panics if `hz` is zero, negative or not finite.
    fn from_hz(hz: f64) -> Self {
        let mut ret = Self::default();
        ret.set_timestep_hz(hz);
        ret
    }

    /// Returns the amount of virtual time that must pass before the fixed
    /// timestep schedule is run again.
    #[inline]
    fn timestep(&self) -> Duration {
        self.context().timestep
    }

    /// Sets the amount of virtual time that must pass before the fixed timestep
    /// schedule is run again, as [`Duration`].
    ///
    /// Takes effect immediately on the next run of the schedule, respecting
    /// what is currently in [`Self::overstep`].
    ///
    /// # Panics
    ///
    /// Panics if `timestep` is zero.
    #[inline]
    fn set_timestep(&mut self, timestep: Duration) {
        assert_ne!(timestep, Duration::ZERO, "attempted to set fixed timestep to zero");
        self.context_mut().timestep = timestep;
    }

    /// Sets the amount of virtual time that must pass before the fixed timestep
    /// schedule is run again, as seconds.
    ///
    /// Timestep is stored as a [`Duration`], which has fixed nanosecond
    /// resolution and will be converted from the floating point number.
    ///
    /// Takes effect immediately on the next run of the schedule, respecting
    /// what is currently in [`Self::overstep`].
    ///
    /// # Panics
    ///
    /// Panics if `seconds` is zero, negative or not finite.
    #[inline]
    fn set_timestep_seconds(&mut self, seconds: f64) {
        assert!(seconds.is_sign_positive(), "seconds less than or equal to zero");
        assert!(seconds.is_finite(), "seconds is infinite");
        self.set_timestep(Duration::from_secs_f64(seconds));
    }

    /// Sets the amount of virtual time that must pass before the fixed timestep
    /// schedule is run again, as frequency.
    ///
    /// The timestep value is set to `1 / hz`, converted to a [`Duration`] which
    /// has fixed nanosecond resolution.
    ///
    /// Takes effect immediately on the next run of the schedule, respecting
    /// what is currently in [`Self::overstep`].
    ///
    /// # Panics
    ///
    /// Panics if `hz` is zero, negative or not finite.
    #[inline]
    fn set_timestep_hz(&mut self, hz: f64) {
        assert!(hz.is_sign_positive(), "Hz less than or equal to zero");
        assert!(hz.is_finite(), "Hz is infinite");
        self.set_timestep_seconds(1.0 / hz);
    }

    /// Returns the amount of overstep time accumulated toward new steps, as
    /// [`Duration`].
    #[inline]
    fn overstep(&self) -> Duration {
        self.context().overstep
    }

    /// Discard a part of the overstep amount.
    ///
    /// If `discard` is higher than overstep, the overstep becomes zero.
    #[inline]
    fn discard_overstep(&mut self, discard: Duration) {
        let context = self.context_mut();
        context.overstep = context.overstep.saturating_sub(discard);
    }

    /// Returns the amount of overstep time accumulated toward new steps, as an
    /// [`f32`] fraction of the timestep.
    #[inline]
    fn overstep_fraction(&self) -> f32 {
        self.context().overstep.as_secs_f32() / self.context().timestep.as_secs_f32()
    }

    /// Returns the amount of overstep time accumulated toward new steps, as an
    /// [`f64`] fraction of the timestep.
    #[inline]
    fn overstep_fraction_f64(&self) -> f64 {
        self.context().overstep.as_secs_f64() / self.context().timestep.as_secs_f64()
    }

    /// Returns a reference to the context of this specific clock.
    fn context(&self) -> &LogicStep;

    /// Returns a mutable reference to the context of this specific clock.
    fn context_mut(&mut self) -> &mut LogicStep;

    /// Advance this clock by adding a `delta` duration to it.
    ///
    /// The added duration will be returned by [`Self::delta`] and
    /// [`Self::elapsed`] will be increased by the duration. Adding
    /// [`Duration::ZERO`] is allowed and will set [`Self::delta`] to zero.
    fn advance_by(&mut self, delta: Duration);

    fn accumulate(&mut self, delta: Duration) {
        self.context_mut().overstep += delta;
    }

    fn expend(&mut self) -> bool {
        let timestep = self.timestep();
        if let Some(new_value) = self.context_mut().overstep.checked_sub(timestep) {
            // reduce accumulated and increase elapsed by period
            self.context_mut().overstep = new_value;
            self.advance_by(timestep);
            true
        } else {
            // no more periods left in accumulated
            false
        }
    }
}
