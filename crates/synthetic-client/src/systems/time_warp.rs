//! Simulation time warp controls and speed ladder.
//!
//! Grounded in SSOT-SYS-000, SSOT-SYS-001, and PLAN-020.

use bevy::prelude::*;

/// Discrete speed presets for simulation time acceleration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum WarpLevel {
    Paused,        // 0x (Frozen time)
    #[default]
    RealTime,      // 1x (1 second / second)
    MinutePerSec,  // 60x (1 minute / second)
    HourPerSec,    // 3,600x (1 hour / second)
    DayPerSec,     // 86,400x (1 day / second)
    MonthPerSec,   // 2,592,000x (1 month / second, 30 days)
}

impl WarpLevel {
    #[allow(dead_code)]
    pub const ALL: [WarpLevel; 6] = [
        WarpLevel::Paused,
        WarpLevel::RealTime,
        WarpLevel::MinutePerSec,
        WarpLevel::HourPerSec,
        WarpLevel::DayPerSec,
        WarpLevel::MonthPerSec,
    ];

    #[must_use]
    pub const fn multiplier(self) -> f64 {
        match self {
            Self::Paused => 0.0,
            Self::RealTime => 1.0,
            Self::MinutePerSec => 60.0,
            Self::HourPerSec => 3600.0,
            Self::DayPerSec => 86400.0,
            Self::MonthPerSec => 2_592_000.0,
        }
    }

    #[must_use]
    #[allow(dead_code)]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Paused => "PAUSED (0x)",
            Self::RealTime => "1 sec / sec (1x)",
            Self::MinutePerSec => "1 min / sec (60x)",
            Self::HourPerSec => "1 hour / sec (3,600x)",
            Self::DayPerSec => "1 day / sec (86,400x)",
            Self::MonthPerSec => "1 month / sec (2,592,000x)",
        }
    }

    #[must_use]
    pub fn step_up(self) -> Self {
        match self {
            Self::Paused => Self::RealTime,
            Self::RealTime => Self::MinutePerSec,
            Self::MinutePerSec => Self::HourPerSec,
            Self::HourPerSec => Self::DayPerSec,
            Self::DayPerSec | Self::MonthPerSec => Self::MonthPerSec,
        }
    }

    #[must_use]
    pub fn step_down(self) -> Self {
        match self {
            Self::Paused | Self::RealTime => Self::Paused,
            Self::MinutePerSec => Self::RealTime,
            Self::HourPerSec => Self::MinutePerSec,
            Self::DayPerSec => Self::HourPerSec,
            Self::MonthPerSec => Self::DayPerSec,
        }
    }
}

/// Simulation time warp manager resource tracking current speed and last non-zero preset.
#[derive(Resource, Debug, Clone, PartialEq)]
pub struct TimeWarp {
    pub current: WarpLevel,
    pub previous_non_zero: WarpLevel,
}

impl Default for TimeWarp {
    fn default() -> Self {
        Self {
            current: WarpLevel::RealTime,
            previous_non_zero: WarpLevel::RealTime,
        }
    }
}

impl TimeWarp {
    pub fn toggle_pause(&mut self) {
        if self.current == WarpLevel::Paused {
            self.current = self.previous_non_zero;
        } else {
            self.previous_non_zero = self.current;
            self.current = WarpLevel::Paused;
        }
    }

    pub fn set_level(&mut self, level: WarpLevel) {
        if level != WarpLevel::Paused {
            self.previous_non_zero = level;
        }
        self.current = level;
    }

    pub fn step_up(&mut self) {
        let next = self.current.step_up();
        self.set_level(next);
    }

    pub fn step_down(&mut self) {
        let prev = self.current.step_down();
        self.set_level(prev);
    }

    #[must_use]
    pub const fn current_multiplier(&self) -> f64 {
        self.current.multiplier()
    }
}

/// Reads keyboard inputs to control simulation time acceleration and pausing.
#[allow(clippy::needless_pass_by_value)]
pub fn keyboard_time_warp_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut time_warp: ResMut<TimeWarp>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        time_warp.toggle_pause();
    } else if keyboard.just_pressed(KeyCode::Digit0) || keyboard.just_pressed(KeyCode::Numpad0) {
        time_warp.set_level(WarpLevel::Paused);
    } else if keyboard.just_pressed(KeyCode::Digit1) || keyboard.just_pressed(KeyCode::Numpad1) {
        time_warp.set_level(WarpLevel::RealTime);
    } else if keyboard.just_pressed(KeyCode::Digit2) || keyboard.just_pressed(KeyCode::Numpad2) {
        time_warp.set_level(WarpLevel::MinutePerSec);
    } else if keyboard.just_pressed(KeyCode::Digit3) || keyboard.just_pressed(KeyCode::Numpad3) {
        time_warp.set_level(WarpLevel::HourPerSec);
    } else if keyboard.just_pressed(KeyCode::Digit4) || keyboard.just_pressed(KeyCode::Numpad4) {
        time_warp.set_level(WarpLevel::DayPerSec);
    } else if keyboard.just_pressed(KeyCode::Digit5) || keyboard.just_pressed(KeyCode::Numpad5) {
        time_warp.set_level(WarpLevel::MonthPerSec);
    } else if keyboard.just_pressed(KeyCode::BracketLeft) {
        time_warp.step_down();
    } else if keyboard.just_pressed(KeyCode::BracketRight) {
        time_warp.step_up();
    }
}

/// Plugin managing simulation time warp inputs and speed scheduling.
pub struct TimeWarpPlugin;

impl Plugin for TimeWarpPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimeWarp>().add_systems(
            Update,
            keyboard_time_warp_system
                .before(crate::systems::astronomy::advance_simulation_time)
                .in_set(crate::systems::astronomy::SimulationTimeSystem),
        );
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn test_warp_multipliers() {
        assert_eq!(WarpLevel::Paused.multiplier(), 0.0);
        assert_eq!(WarpLevel::RealTime.multiplier(), 1.0);
        assert_eq!(WarpLevel::MinutePerSec.multiplier(), 60.0);
        assert_eq!(WarpLevel::HourPerSec.multiplier(), 3600.0);
        assert_eq!(WarpLevel::DayPerSec.multiplier(), 86400.0);
        assert_eq!(WarpLevel::MonthPerSec.multiplier(), 2_592_000.0);
    }

    #[test]
    fn test_warp_stepping_and_clamping() {
        let mut warp = TimeWarp::default();
        assert_eq!(warp.current, WarpLevel::RealTime);

        warp.step_up();
        assert_eq!(warp.current, WarpLevel::MinutePerSec);
        warp.step_up();
        assert_eq!(warp.current, WarpLevel::HourPerSec);
        warp.step_up();
        assert_eq!(warp.current, WarpLevel::DayPerSec);
        warp.step_up();
        assert_eq!(warp.current, WarpLevel::MonthPerSec);
        // Clamping at upper bound
        warp.step_up();
        assert_eq!(warp.current, WarpLevel::MonthPerSec);

        // Step back down
        warp.step_down();
        assert_eq!(warp.current, WarpLevel::DayPerSec);
        warp.step_down();
        assert_eq!(warp.current, WarpLevel::HourPerSec);
        warp.step_down();
        assert_eq!(warp.current, WarpLevel::MinutePerSec);
        warp.step_down();
        assert_eq!(warp.current, WarpLevel::RealTime);
        warp.step_down();
        assert_eq!(warp.current, WarpLevel::Paused);
        // Clamping at lower bound
        warp.step_down();
        assert_eq!(warp.current, WarpLevel::Paused);
    }

    #[test]
    fn test_pause_toggle_memory() {
        let mut warp = TimeWarp::default();
        assert_eq!(warp.current, WarpLevel::RealTime);
        assert_eq!(warp.previous_non_zero, WarpLevel::RealTime);

        warp.set_level(WarpLevel::DayPerSec);
        assert_eq!(warp.current, WarpLevel::DayPerSec);
        assert_eq!(warp.previous_non_zero, WarpLevel::DayPerSec);

        // Toggle pause via Space
        warp.toggle_pause();
        assert_eq!(warp.current, WarpLevel::Paused);
        assert_eq!(warp.previous_non_zero, WarpLevel::DayPerSec);

        // Explicitly setting paused or stepping down must not clobber previous_non_zero
        warp.set_level(WarpLevel::Paused);
        assert_eq!(warp.previous_non_zero, WarpLevel::DayPerSec);
        warp.step_down();
        assert_eq!(warp.previous_non_zero, WarpLevel::DayPerSec);

        // Toggle resume via Space
        warp.toggle_pause();
        assert_eq!(warp.current, WarpLevel::DayPerSec);
    }
}
