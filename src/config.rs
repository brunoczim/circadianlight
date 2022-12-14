//! Configuration for the execution of the program, including configuration for
//! the day phases, as well for channels minimum and maximum.

use std::{error::Error, fmt};

/// Error yielded when an invalid day phase cycle is given during a
/// [`HourConfig`].
#[derive(Debug, Clone)]
pub struct InvalidDayPhases {
    /// Starting hour of the day phase, divided por 24h (in the interval
    /// `[0,1)`).
    pub day_start: f64,
    /// Starting hour of the dusk phase, divided por 24h (in the interval
    /// `[0,1)`).
    pub dusk_start: f64,
    /// Starting hour of the night phase, divided por 24h (in the interval
    /// `[0,1)`).
    pub night_start: f64,
}

impl fmt::Display for InvalidDayPhases {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "Invalid day phases sequence, expected a cycle of day -> dusk -> \
             night -> day; on an interval [0.0, 1.0), given day start: {}, \
             dusk start: {}, night start: {}",
            self.day_start, self.dusk_start, self.night_start
        )
    }
}

impl Error for InvalidDayPhases {}

/// Error yielded when invalid channel bounds are found during the creation of a
/// [`ChannelConfig`].
#[derive(Debug, Clone)]
pub struct InvalidChannelBounds {
    /// Given minimum value for the channel (in the interval `[0,1]`).
    pub min: f64,
    /// Given maximum value for the channel (in the interval `[0,1]`).
    pub max: f64,
}

impl fmt::Display for InvalidChannelBounds {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "Invalid color channel bounds, expected min <= max; on an \
             interval [0.0, 1.0), given min: {}, max: {}",
            self.min, self.max
        )
    }
}

impl Error for InvalidChannelBounds {}

/// Configuration used for customizing starting hour of each day phase. See
/// [`DayPhase`](crate::hour::DayPhase).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct HourConfig {
    day_start: f64,
    dusk_start: f64,
    night_start: f64,
}

impl Default for HourConfig {
    fn default() -> Self {
        Self {
            day_start: 5.0 / 24.0,
            dusk_start: 17.0 / 24.0,
            night_start: 21.0 / 24.0,
        }
    }
}

impl HourConfig {
    /// Creates a new hour configuration, given hour of the day phase start,
    /// hour of the dusk phase start and hour of the night phase start,
    /// compressed in the interval `[0,1)`, i.e. divided by `24h`.
    /// Note that, wraping around `24h`, the order `day -> dusk -> night -> day`
    /// should be respected, i.e. `day <= dusk <= night`, `dusk <= night <=
    /// day`, `night <= day <= dusk` are valid, but otherwise not.
    pub fn new(
        day_start: f64,
        dusk_start: f64,
        night_start: f64,
    ) -> Result<Self, InvalidDayPhases> {
        if day_start <= dusk_start && dusk_start <= night_start
            || night_start <= day_start && day_start <= dusk_start
            || dusk_start <= night_start && night_start <= day_start
        {
            Ok(Self { day_start, dusk_start, night_start })
        } else {
            Err(InvalidDayPhases { day_start, dusk_start, night_start })
        }
    }

    /// Starting hour of the day phase, compressed `24h` into the interval
    /// `[0,1)`.
    pub fn day_start(self) -> f64 {
        self.day_start
    }

    /// Starting hour of the dusk phase, compressed `24h` into the interval
    /// `[0,1)`.
    pub fn dusk_start(self) -> f64 {
        self.dusk_start
    }

    /// Starting hour of the night phase, compressed `24h` into the interval
    /// `[0,1)`.
    pub fn night_start(self) -> f64 {
        self.night_start
    }
}

/// Configuration of a color channel.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ChannelConfig {
    min: f64,
    max: f64,
}

impl Default for ChannelConfig {
    fn default() -> Self {
        Self { min: 0.0, max: 1.0 }
    }
}

impl ChannelConfig {
    /// Creates the configuration of a color channel, given its minimum and
    /// maximum value in the interval `[0,1]`.
    /// Note that `minimum <= maximum` is a restriction that must be respected.
    pub fn new(
        min: f64,
        max: f64,
    ) -> Result<ChannelConfig, InvalidChannelBounds> {
        if min <= max {
            Ok(Self { min, max })
        } else {
            Err(InvalidChannelBounds { min, max })
        }
    }

    /// Minimum value of a color channel in the interval `[0,1]`.
    pub fn min(self) -> f64 {
        self.min
    }

    /// Maximum value of a color channel in the interval `[0,1]`.
    pub fn max(self) -> f64 {
        self.max
    }
}

/// General configuration of the application.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Config {
    /// Configuration of day phases.
    pub hours: HourConfig,
    /// Configuration of color channels, in the order: red, green, blue.
    pub channels: [ChannelConfig; 3],
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hours: HourConfig::default(),
            channels: [
                ChannelConfig { min: 1.0, max: 1.0 },
                ChannelConfig { min: 0.65, max: 1.0 },
                ChannelConfig { min: 0.45, max: 1.0 },
            ],
        }
    }
}

#[cfg(test)]
mod test {
    use super::{ChannelConfig, HourConfig};

    #[test]
    fn error_when_day_phase_cycle_is_invalid() {
        HourConfig::new(0.5, 0.1, 0.7).unwrap_err();
        HourConfig::new(0.1, 0.7, 0.5).unwrap_err();
        HourConfig::new(0.7, 0.5, 0.1).unwrap_err();
    }

    #[test]
    fn ok_when_day_phase_cycle_is_valid() {
        HourConfig::new(0.1, 0.5, 0.7).unwrap();
        HourConfig::new(0.5, 0.7, 0.1).unwrap();
        HourConfig::new(0.7, 0.1, 0.5).unwrap();
    }

    #[test]
    fn error_when_channel_bounds_are_invalid() {
        ChannelConfig::new(0.9, 0.1).unwrap_err();
    }

    #[test]
    fn ok_when_channel_bounds_are_valid() {
        ChannelConfig::new(0.1, 0.9).unwrap();
        ChannelConfig::new(1.0, 1.0).unwrap();
    }
}
