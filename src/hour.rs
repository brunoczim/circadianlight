//! Utilities related to day phases.

use crate::config::HourConfig;
use chrono::Timelike;

/// Converts a `chrono` time-like object into a compressed `24h` day hour in the
/// interval `[0,1)`.
pub fn timelike_to_hours<T>(timelike: &T) -> f64
where
    T: Timelike + ?Sized,
{
    let seconds = f64::from(timelike.num_seconds_from_midnight());
    let nanoseconds_frac = f64::from(timelike.nanosecond()) / 1_000_000_000.0;
    let nanoseconds = seconds + nanoseconds_frac;
    nanoseconds / (60.0 * 60.0 * 24.0)
}

/// A day phase.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum DayPhase {
    /// Any part of the bright sun day phase.
    Day,
    /// A part of the transition from day to night, with an indication of where
    /// this transition is, given `0` for just after day, `1` for starting
    /// night (i.e. in the interval `[0, 1)`.
    Dusk(f64),
    /// Any part of the night phase.
    Night,
}

impl DayPhase {
    /// Computes the day phase given a day phase hour configuration and the
    /// current hour compressed in the interval `[0,1)` (where `1 = 24h`).
    pub fn from_current_hour(
        hour_config: HourConfig,
        current_hour: f64,
    ) -> Self {
        if hour_config.day_start() <= hour_config.dusk_start()
            && hour_config.dusk_start() <= hour_config.night_start()
        {
            if current_hour >= hour_config.day_start()
                && current_hour < hour_config.dusk_start()
            {
                Self::Day
            } else if current_hour >= hour_config.dusk_start()
                && current_hour < hour_config.night_start()
            {
                Self::Dusk(
                    (current_hour - hour_config.dusk_start())
                        / (hour_config.night_start()
                            - hour_config.dusk_start()),
                )
            } else {
                Self::Night
            }
        } else if hour_config.night_start() <= hour_config.day_start()
            && hour_config.day_start() <= hour_config.dusk_start()
        {
            if current_hour >= hour_config.day_start()
                && current_hour < hour_config.dusk_start()
            {
                Self::Day
            } else if current_hour >= hour_config.dusk_start() {
                Self::Dusk(
                    (current_hour - hour_config.dusk_start())
                        / (1.0 + hour_config.night_start()
                            - hour_config.dusk_start()),
                )
            } else if current_hour < hour_config.night_start() {
                Self::Dusk(
                    (1.0 + current_hour - hour_config.dusk_start())
                        / (1.0 + hour_config.night_start()
                            - hour_config.dusk_start()),
                )
            } else {
                Self::Night
            }
        } else if hour_config.dusk_start() <= hour_config.night_start()
            && hour_config.night_start() <= hour_config.day_start()
        {
            if current_hour >= hour_config.day_start()
                || current_hour < hour_config.dusk_start()
            {
                Self::Day
            } else if current_hour >= hour_config.dusk_start()
                && current_hour < hour_config.night_start()
            {
                Self::Dusk(
                    (current_hour - hour_config.dusk_start())
                        / (hour_config.night_start()
                            - hour_config.dusk_start()),
                )
            } else {
                Self::Night
            }
        } else {
            panic!("Incorrect hour configuration")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::config::HourConfig;

    use super::DayPhase;

    const EPSILON: f64 = 0.01;

    #[test]
    fn day_phase_from_current_hour_is_day() {
        assert_eq!(
            DayPhase::from_current_hour(HourConfig::default(), 12.0 / 24.0),
            DayPhase::Day,
        );
    }

    #[test]
    fn day_phase_from_current_hour_is_dusk() {
        match DayPhase::from_current_hour(HourConfig::default(), 19.0 / 24.0) {
            DayPhase::Dusk(scale) => {
                assert!((scale - (19.0 - 17.0) / (21.0 - 17.0)).abs() < EPSILON)
            },
            value => panic!("Expected dusk, found {:?}", value),
        }
    }

    #[test]
    fn day_phase_from_current_hour_is_night() {
        assert_eq!(
            DayPhase::from_current_hour(HourConfig::default(), 23.0 / 24.0),
            DayPhase::Night,
        );
        assert_eq!(
            DayPhase::from_current_hour(HourConfig::default(), 1.0 / 24.0),
            DayPhase::Night,
        );
    }

    #[test]
    fn chaotic_day_phae_order() {
        let config =
            HourConfig::new(10.0 / 24.0, 19.0 / 24.0, 1.0 / 24.0).unwrap();
        assert_eq!(
            DayPhase::from_current_hour(config, 1.1 / 24.0),
            DayPhase::Night,
        );
        assert_eq!(
            DayPhase::from_current_hour(config, 5.0 / 24.0),
            DayPhase::Night,
        );
        assert_eq!(
            DayPhase::from_current_hour(config, 11.0 / 24.0),
            DayPhase::Day,
        );
        assert!(matches!(
            DayPhase::from_current_hour(config, 0.0 / 24.0),
            DayPhase::Dusk(_)
        ));
    }
}
