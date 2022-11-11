use chrono::Timelike;

pub const RED_CHANNEL: usize = 0;
pub const GREEN_CHANNEL: usize = 1;
pub const BLUE_CHANNEL: usize = 2;

pub fn timelike_to_hours<T>(timelike: &T) -> f64
where
    T: Timelike + ?Sized,
{
    let seconds = f64::from(timelike.num_seconds_from_midnight());
    let nanoseconds_frac = f64::from(timelike.nanosecond()) / 1_000_000_000.0;
    let nanoseconds = seconds + nanoseconds_frac;
    nanoseconds / (60.0 * 60.0 * 24.0)
}

pub fn map_channel_vector<F, T, U>(input: [T; 3], mut mapper: F) -> [U; 3]
where
    T: Copy,
    F: FnMut(T) -> U,
{
    [mapper(input[0]), mapper(input[1]), mapper(input[2])]
}

pub fn linear_channel_function(
    channel_config: ChannelConfig,
    hour_config: HourConfig,
    current_hour: f64,
) -> f64 {
    match DayPhase::from_current_hour(hour_config, current_hour) {
        DayPhase::Day => channel_config.max,
        DayPhase::Night => channel_config.min,
        DayPhase::Dusk(scale) => {
            channel_config.min
                + (channel_config.max - channel_config.min) * (1.0 - scale)
        },
    }
}

pub fn gamma_function(
    config: Config,
) -> impl Fn(f64) -> [f64; 3] + Copy + Send + Sync + 'static {
    move |current_hour| {
        map_channel_vector(config.channels, |channel_config| {
            linear_channel_function(channel_config, config.hours, current_hour)
        })
    }
}

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
        if hour_config.day_start <= hour_config.dusk_start
            && hour_config.dusk_start <= hour_config.night_start
        {
            if current_hour >= hour_config.day_start
                && current_hour <= hour_config.dusk_start
            {
                Self::Day
            } else if current_hour >= hour_config.dusk_start
                && current_hour <= hour_config.night_start
            {
                Self::Dusk(
                    (current_hour - hour_config.dusk_start)
                        / (hour_config.night_start - hour_config.dusk_start),
                )
            } else {
                Self::Night
            }
        } else if hour_config.night_start <= hour_config.day_start
            && hour_config.day_start <= hour_config.dusk_start
        {
            if current_hour >= hour_config.day_start
                && current_hour <= hour_config.dusk_start
            {
                Self::Day
            } else if current_hour >= hour_config.dusk_start {
                Self::Dusk(
                    (current_hour - hour_config.dusk_start)
                        / (1.0 + hour_config.night_start
                            - hour_config.dusk_start),
                )
            } else if current_hour <= hour_config.night_start {
                Self::Dusk(
                    (1.0 + current_hour - hour_config.dusk_start)
                        / (1.0 + hour_config.night_start
                            - hour_config.dusk_start),
                )
            } else {
                Self::Night
            }
        } else if hour_config.dusk_start <= hour_config.night_start
            && hour_config.night_start <= hour_config.day_start
        {
            if current_hour >= hour_config.day_start
                || current_hour <= hour_config.dusk_start
            {
                Self::Day
            } else if current_hour >= hour_config.dusk_start
                && current_hour <= hour_config.night_start
            {
                Self::Dusk(
                    (current_hour - hour_config.dusk_start)
                        / (hour_config.night_start - hour_config.dusk_start),
                )
            } else {
                Self::Night
            }
        } else {
            panic!("Incorrect hour configuration")
        }
    }
}
