use crate::{
    config::{ChannelConfig, Config, HourConfig},
    hour::DayPhase,
};

pub const RED_CHANNEL: usize = 0;
pub const GREEN_CHANNEL: usize = 1;
pub const BLUE_CHANNEL: usize = 2;

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
    let min = channel_config.min();
    let max = channel_config.max();
    match DayPhase::from_current_hour(hour_config, current_hour) {
        DayPhase::Day => max,
        DayPhase::Night => min,
        DayPhase::Dusk(scale) => min + (max - min) * (1.0 - scale),
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
