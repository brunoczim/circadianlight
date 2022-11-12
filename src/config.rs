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

    pub fn with_day_start(self, day_start: f64) -> Self {
        assert!(
            day_start <= self.dusk_start && self.dusk_start <= self.night_start
                || self.night_start <= day_start
                    && day_start <= self.dusk_start
                || self.dusk_start <= self.night_start
                    && self.night_start <= day_start,
            "Given day start {} violates day phases order: day -> dusk {} -> \
             night {}",
            day_start,
            self.dusk_start,
            self.night_start
        );
        Self { day_start, ..self }
    }

    pub fn with_dusk_start(self, dusk_start: f64) -> Self {
        assert!(
            self.day_start <= dusk_start && dusk_start <= self.night_start
                || self.night_start <= self.day_start
                    && self.day_start <= dusk_start
                || dusk_start <= self.night_start
                    && self.night_start <= self.day_start,
            "Given dusk start {} violates day phases order: day {} -> dusk  \
             -> night {}",
            dusk_start,
            self.day_start,
            self.night_start,
        );
        Self { dusk_start, ..self }
    }

    pub fn with_night_start(self, night_start: f64) -> Self {
        assert!(
            self.day_start <= self.dusk_start && self.dusk_start <= night_start
                || night_start <= self.day_start
                    && self.day_start <= self.dusk_start
                || self.dusk_start <= night_start
                    && night_start <= self.day_start,
            "Given night start {} violates day phases order: day {} -> dusk \
             {} -> night",
            self.day_start,
            self.dusk_start,
            night_start
        );
        Self { night_start, ..self }
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
    pub fn with_min(self, min: f64) -> Self {
        assert!(
            min <= self.max,
            "Minimum value {} for channel is bigger than maximum {}",
            min,
            self.max
        );
        Self { min, ..self }
    }

    pub fn with_max(self, max: f64) -> Self {
        assert!(
            max >= self.min,
            "Maximum value {} for channel is bigger than minimum {}",
            max,
            self.min
        );
        Self { max, ..self }
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
