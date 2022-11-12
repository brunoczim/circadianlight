use crate::config::HourConfig;
use chrono::Timelike;

pub fn timelike_to_hours<T>(timelike: &T) -> f64
where
    T: Timelike + ?Sized,
{
    let seconds = f64::from(timelike.num_seconds_from_midnight());
    let nanoseconds_frac = f64::from(timelike.nanosecond()) / 1_000_000_000.0;
    let nanoseconds = seconds + nanoseconds_frac;
    nanoseconds / (60.0 * 60.0 * 24.0)
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum DayPhase {
    Day,
    Dusk(f64),
    Night,
}

impl DayPhase {
    pub fn from_current_hour(
        hour_config: HourConfig,
        current_hour: f64,
    ) -> Self {
        if hour_config.day_start() <= hour_config.dusk_start()
            && hour_config.dusk_start() <= hour_config.night_start()
        {
            if current_hour >= hour_config.day_start()
                && current_hour <= hour_config.dusk_start()
            {
                Self::Day
            } else if current_hour >= hour_config.dusk_start()
                && current_hour <= hour_config.night_start()
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
                && current_hour <= hour_config.dusk_start()
            {
                Self::Day
            } else if current_hour >= hour_config.dusk_start() {
                Self::Dusk(
                    (current_hour - hour_config.dusk_start())
                        / (1.0 + hour_config.night_start()
                            - hour_config.dusk_start()),
                )
            } else if current_hour <= hour_config.night_start() {
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
                || current_hour <= hour_config.dusk_start()
            {
                Self::Day
            } else if current_hour >= hour_config.dusk_start()
                && current_hour <= hour_config.night_start()
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
