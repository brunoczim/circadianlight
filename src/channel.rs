//! Utilites for color channels.

use crate::{
    config::{ChannelConfig, Config, HourConfig},
    hour::DayPhase,
};

/// Index of the red color channel, never intended to change.
pub const RED: usize = 0;
/// Index of the green color channel, never intended to change.
pub const GREEN: usize = 1;
/// Index of the blue color channel, never intended to change.
pub const BLUE: usize = 2;

/// Maps channels of different types.
pub fn map_channel_vector<F, T, U>(input: [T; 3], mut mapper: F) -> [U; 3]
where
    T: Copy,
    F: FnMut(T) -> U,
{
    [mapper(input[0]), mapper(input[1]), mapper(input[2])]
}

/// Creates a linear channel function, where linear refers that the dusk is
/// processed with a linear function.
pub fn linear_channel_function(
    channel_config: ChannelConfig,
    hour_config: HourConfig,
) -> impl Fn(f64) -> f64 + Copy + Send + Sync + 'static {
    let min = channel_config.min();
    let max = channel_config.max();
    move |current_hour| match DayPhase::from_current_hour(
        hour_config,
        current_hour,
    ) {
        DayPhase::Day => max,
        DayPhase::Night => min,
        DayPhase::Dusk(scale) => min + (max - min) * (1.0 - scale),
    }
}

/// Creates a gamma function that adapts to the hour of the day.
pub fn gamma_function(
    config: Config,
) -> impl Fn(f64) -> [f64; 3] + Copy + Send + Sync + 'static {
    move |current_hour| {
        map_channel_vector(config.channels, |channel_config| {
            linear_channel_function(channel_config, config.hours)(current_hour)
        })
    }
}

#[cfg(test)]
mod test {
    use crate::config::{ChannelConfig, HourConfig};

    use super::linear_channel_function;

    const EPSILON: f64 = 0.01;

    #[test]
    fn linear_channel_function_on_day() {
        let channel = linear_channel_function(
            ChannelConfig::new(0.4, 0.9).unwrap(),
            HourConfig::default(),
        )(12.0 / 24.0);
        assert!((channel - 0.9).abs() < EPSILON);

        let channel = linear_channel_function(
            ChannelConfig::new(1.0, 1.0).unwrap(),
            HourConfig::default(),
        )(9.0 / 24.0);
        assert!((channel - 1.0).abs() < EPSILON);

        let channel = linear_channel_function(
            ChannelConfig::new(0.8, 1.0).unwrap(),
            HourConfig::default(),
        )(9.0 / 24.0);
        assert!((channel - 1.0).abs() < EPSILON);
    }

    #[test]
    fn linear_channel_function_on_dusk() {
        let channel = linear_channel_function(
            ChannelConfig::new(0.4, 0.9).unwrap(),
            HourConfig::default(),
        )(19.0 / 24.0);
        assert!(channel > 0.4 + EPSILON && channel < 0.9 - EPSILON);

        let channel = linear_channel_function(
            ChannelConfig::new(1.0, 1.0).unwrap(),
            HourConfig::default(),
        )(19.0 / 24.0);
        assert!((channel - 1.0).abs() < EPSILON);

        let channel = linear_channel_function(
            ChannelConfig::new(0.8, 1.0).unwrap(),
            HourConfig::default(),
        )(19.0 / 24.0);
        assert!(channel > 0.8 + EPSILON && channel < 1.0 - EPSILON);
    }

    #[test]
    fn linear_channel_function_on_night() {
        let channel = linear_channel_function(
            ChannelConfig::new(0.4, 0.9).unwrap(),
            HourConfig::default(),
        )(23.0 / 24.0);
        assert!((channel - 0.4).abs() < EPSILON);

        let channel = linear_channel_function(
            ChannelConfig::new(0.4, 0.9).unwrap(),
            HourConfig::default(),
        )(1.0 / 24.0);
        assert!((channel - 0.4).abs() < EPSILON);

        let channel = linear_channel_function(
            ChannelConfig::new(1.0, 1.0).unwrap(),
            HourConfig::default(),
        )(1.0 / 24.0);
        assert!((channel - 1.0).abs() < EPSILON);

        let channel = linear_channel_function(
            ChannelConfig::new(0.8, 1.0).unwrap(),
            HourConfig::default(),
        )(1.0 / 24.0);
        assert!((channel - 0.8).abs() < EPSILON);
    }
}
