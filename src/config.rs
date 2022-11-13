use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct InvalidDayPhases {
    pub day_start: f64,
    pub dusk_start: f64,
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

#[derive(Debug, Clone)]
pub struct InvalidChannelBounds {
    pub min: f64,
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
    pub fn day_start(self) -> f64 {
        self.day_start
    }

    pub fn dusk_start(self) -> f64 {
        self.dusk_start
    }

    pub fn night_start(self) -> f64 {
        self.night_start
    }

    pub fn with_day_start(
        self,
        day_start: f64,
    ) -> Result<Self, InvalidDayPhases> {
        if day_start <= self.dusk_start && self.dusk_start <= self.night_start
            || self.night_start <= day_start && day_start <= self.dusk_start
            || self.dusk_start <= self.night_start
                && self.night_start <= day_start
        {
            Ok(Self { day_start, ..self })
        } else {
            Err(InvalidDayPhases {
                day_start,
                dusk_start: self.dusk_start,
                night_start: self.night_start,
            })
        }
    }

    pub fn with_dusk_start(
        self,
        dusk_start: f64,
    ) -> Result<HourConfig, InvalidDayPhases> {
        if self.day_start <= dusk_start && dusk_start <= self.night_start
            || self.night_start <= self.day_start
                && self.day_start <= dusk_start
            || dusk_start <= self.night_start
                && self.night_start <= self.day_start
        {
            Ok(Self { dusk_start, ..self })
        } else {
            Err(InvalidDayPhases {
                day_start: self.day_start,
                dusk_start,
                night_start: self.night_start,
            })
        }
    }

    pub fn with_night_start(
        self,
        night_start: f64,
    ) -> Result<HourConfig, InvalidDayPhases> {
        if self.day_start <= self.dusk_start && self.dusk_start <= night_start
            || night_start <= self.day_start
                && self.day_start <= self.dusk_start
            || self.dusk_start <= night_start && night_start <= self.day_start
        {
            Ok(Self { night_start, ..self })
        } else {
            Err(InvalidDayPhases {
                day_start: self.day_start,
                dusk_start: self.dusk_start,
                night_start,
            })
        }
    }
}

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
    pub fn with_min(
        self,
        min: f64,
    ) -> Result<ChannelConfig, InvalidChannelBounds> {
        if min <= self.max {
            Ok(Self { min, ..self })
        } else {
            Err(InvalidChannelBounds { min, max: self.max })
        }
    }

    pub fn with_max(
        self,
        max: f64,
    ) -> Result<ChannelConfig, InvalidChannelBounds> {
        if self.min <= max {
            Ok(Self { max, ..self })
        } else {
            Err(InvalidChannelBounds { min: self.min, max })
        }
    }

    pub fn min(self) -> f64 {
        self.min
    }

    pub fn max(self) -> f64 {
        self.max
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Config {
    pub hours: HourConfig,
    pub channels: [ChannelConfig; 3],
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hours: HourConfig::default(),
            channels: [
                ChannelConfig { min: 1.0, max: 1.0 },
                ChannelConfig { min: 0.8, max: 1.0 },
                ChannelConfig { min: 0.6, max: 1.0 },
            ],
        }
    }
}
