//! CLI (Command-Line Interface) utilites.

use std::{io, thread, time::Duration};

use chrono::{Local, NaiveTime};
use structopt::StructOpt;

use crate::{
    channel::{self, gamma_function},
    config::{
        ChannelConfig,
        Config,
        HourConfig,
        InvalidChannelBounds,
        InvalidDayPhases,
    },
    environment::{GraphicalEnv, GraphicalEnvContext},
    hour::timelike_to_hours,
};

/// Common args for configuring the gamma funcion.
#[derive(Debug, Clone, StructOpt)]
pub struct ConfigArgs {
    /// Minimum red channel value, in the interval `[0,1]`.
    #[structopt(long = "--min-red")]
    #[structopt(short = "-r")]
    #[structopt(default_value = "1.0")]
    min_red: f64,
    /// Maximum red channel value, in the interval `[0,1]`.
    #[structopt(long = "--max-red")]
    #[structopt(short = "-R")]
    #[structopt(default_value = "1.0")]
    max_red: f64,
    /// Minimum green channel value, in the interval `[0,1]`.
    #[structopt(long = "--min-green")]
    #[structopt(short = "-g")]
    #[structopt(default_value = "0.6")]
    min_green: f64,
    /// Maximum green channel value, in the interval `[0,1]`.
    #[structopt(long = "--max-green")]
    #[structopt(short = "-G")]
    #[structopt(default_value = "1.0")]
    max_green: f64,
    /// Minimum blue channel value, in the interval `[0,1]`.
    #[structopt(long = "--min-blue")]
    #[structopt(short = "-b")]
    #[structopt(default_value = "0.3")]
    min_blue: f64,
    /// Maximum blue channel value, in the interval `[0,1]`.
    #[structopt(long = "--max-blue")]
    #[structopt(short = "-B")]
    #[structopt(default_value = "1.0")]
    max_blue: f64,
    /// Starting hour of the day phase.
    #[structopt(long = "--day-start")]
    #[structopt(short = "-d")]
    #[structopt(default_value = "05:00")]
    #[structopt(parse(try_from_str = parse_time_arg))]
    day_start: NaiveTime,
    /// Starting hour of the dusk phase.
    #[structopt(long = "--dusk-start")]
    #[structopt(short = "-D")]
    #[structopt(default_value = "17:00")]
    #[structopt(parse(try_from_str = parse_time_arg))]
    dusk_start: NaiveTime,
    /// Starting hour of the night phase.
    #[structopt(long = "--night-start")]
    #[structopt(short = "-n")]
    #[structopt(default_value = "21:00")]
    #[structopt(parse(try_from_str = parse_time_arg))]
    night_start: NaiveTime,
}

impl ConfigArgs {
    /// Creates an hour configuration from these args.
    pub fn create_hour_config(&self) -> Result<HourConfig, InvalidDayPhases> {
        HourConfig::new(
            timelike_to_hours(&self.day_start),
            timelike_to_hours(&self.dusk_start),
            timelike_to_hours(&self.night_start),
        )
    }

    /// Creates channels' configurations from these args.
    pub fn create_channels_config(
        &self,
    ) -> Result<[ChannelConfig; 3], InvalidChannelBounds> {
        Ok([
            ChannelConfig::new(self.min_red, self.max_red)?,
            ChannelConfig::new(self.min_green, self.max_green)?,
            ChannelConfig::new(self.min_blue, self.max_blue)?,
        ])
    }

    /// Creates whole configuration from these args.
    pub fn create_config(&self) -> io::Result<Config> {
        let hours = self.create_hour_config().map_err(|error| {
            io::Error::new(io::ErrorKind::InvalidInput, error)
        })?;
        let channels = self.create_channels_config().map_err(|error| {
            io::Error::new(io::ErrorKind::InvalidInput, error)
        })?;
        Ok(Config { hours, channels })
    }
}

/// Circadian Light is a program controls the color spectrum of your screen
/// according to the current day time in order to improve the quality of your
/// sleep.
/// The more night it becomes, the more red your computer screen will emit
/// (actually, it is more correct to say that it will emit less green and blue
/// light).
#[derive(Debug, Clone, StructOpt)]
#[structopt(version = "0.1")]
pub struct Program {
    #[structopt(subcommand)]
    subcommand: SubCommand,
}

impl GraphicalEnvContext for Program {
    type Output = ();

    fn with_graphical_env<G>(self, graphical_env: G) -> io::Result<Self::Output>
    where
        G: GraphicalEnv,
    {
        self.subcommand.with_graphical_env(graphical_env)
    }

    fn without_graphical_env(self) -> io::Result<Self::Output> {
        self.subcommand.without_graphical_env()
    }
}

/// Subcommand required to run circadianlight.
#[derive(Debug, Clone, StructOpt)]
pub enum SubCommand {
    /// Run it as a service from minute to minute or in the desired interval.
    Serve(ServeSubCommand),
    /// Just prints the color spectrum for the current hour (or the given
    /// hour).
    Print(PrintSubCommand),
    /// Applies once the color spectrum to the screen according to current hour
    /// (or the given hour).
    Apply(ApplySubCommand),
}

impl GraphicalEnvContext for SubCommand {
    type Output = ();

    fn with_graphical_env<G>(self, graphical_env: G) -> io::Result<Self::Output>
    where
        G: GraphicalEnv,
    {
        match self {
            Self::Serve(subcommand) => {
                subcommand.with_graphical_env(graphical_env)
            },
            Self::Print(subcommand) => {
                subcommand.with_graphical_env(graphical_env)
            },
            Self::Apply(subcommand) => {
                subcommand.with_graphical_env(graphical_env)
            },
        }
    }

    fn without_graphical_env(self) -> io::Result<Self::Output> {
        match self {
            Self::Serve(subcommand) => subcommand.without_graphical_env(),
            Self::Print(subcommand) => subcommand.without_graphical_env(),
            Self::Apply(subcommand) => subcommand.without_graphical_env(),
        }
    }
}

/// Run it as a service, running minute to minute or in the desired interval.
#[derive(Debug, Clone, StructOpt)]
pub struct ServeSubCommand {
    /// Seconds to wait beetween every update to screen colors.
    #[structopt(long = "--sleep-seconds")]
    #[structopt(short = "-s")]
    #[structopt(default_value = "60")]
    sleep_seconds: u64,
    /// List of currently used monitors. If not given, it will be obtained from
    /// your graphical environment, and all of currently used monitors will
    /// be targetted.
    #[structopt(long = "--monitors")]
    #[structopt(short = "-m")]
    monitors: Option<Vec<String>>,
    /// Arguments for configuration of the gamma function.
    #[structopt(flatten)]
    config_args: ConfigArgs,
}

impl GraphicalEnvContext for ServeSubCommand {
    type Output = ();

    fn with_graphical_env<G>(self, graphical_env: G) -> io::Result<Self::Output>
    where
        G: GraphicalEnv,
    {
        let config = self.config_args.create_config()?;
        loop {
            let gamma = create_color_channels(config, None);
            match &self.monitors {
                Some(monitors) => {
                    graphical_env.apply_gamma(gamma, monitors)?;
                },
                None => {
                    let monitors = graphical_env.list_monitors()?;
                    graphical_env.apply_gamma(gamma, monitors)?;
                },
            }
            thread::sleep(Duration::from_secs(self.sleep_seconds));
        }
    }
}

/// Just prints the color spectrum for the current hour (or the given
/// hour).
#[derive(Debug, Clone, StructOpt)]
pub struct PrintSubCommand {
    /// The time in format `H:M` from which the colors will be computed. If not
    /// given, the current hour and minute is given.
    #[structopt(long = "--time")]
    #[structopt(short = "-t")]
    #[structopt(parse(try_from_str = parse_time_arg))]
    time: Option<NaiveTime>,
    /// Arguments for configuration of the gamma function.
    #[structopt(flatten)]
    config_args: ConfigArgs,
}

impl GraphicalEnvContext for PrintSubCommand {
    type Output = ();

    fn with_graphical_env<G>(self, graphical_env: G) -> io::Result<Self::Output>
    where
        G: GraphicalEnv,
    {
        let config = self.config_args.create_config()?;
        let gamma = create_color_channels(config, self.time);
        println!("{}", graphical_env.format_gamma(gamma)?);
        Ok(())
    }

    fn without_graphical_env(self) -> io::Result<Self::Output> {
        let config = self.config_args.create_config()?;
        let gamma = create_color_channels(config, self.time);
        println!(
            "red={:.3} green={:.3} blue={:.3}",
            gamma[channel::RED],
            gamma[channel::GREEN],
            gamma[channel::BLUE],
        );
        Ok(())
    }
}

/// Applies once the color spectrum to the screen according to current hour
/// (or the given hour).
#[derive(Debug, Clone, StructOpt)]
pub struct ApplySubCommand {
    /// The time in format `H:M` from which the colors will be computed. If not
    /// given, the current hour and minute is given.
    #[structopt(long = "--time")]
    #[structopt(short = "-t")]
    #[structopt(parse(try_from_str = parse_time_arg))]
    time: Option<NaiveTime>,
    /// List of currently used monitors. If not given, it will be obtained from
    /// your graphical environment, and all of currently used monitors will
    /// be targetted.
    #[structopt(long = "--monitors")]
    #[structopt(short = "-m")]
    monitors: Option<Vec<String>>,
    /// Arguments for configuration of the gamma function.
    #[structopt(flatten)]
    config_args: ConfigArgs,
}

impl GraphicalEnvContext for ApplySubCommand {
    type Output = ();

    fn with_graphical_env<G>(self, graphical_env: G) -> io::Result<Self::Output>
    where
        G: GraphicalEnv,
    {
        let config = self.config_args.create_config()?;
        let gamma = create_color_channels(config, self.time);
        match self.monitors {
            Some(monitors) => {
                graphical_env.apply_gamma(gamma, monitors)?;
            },
            None => {
                let monitors = graphical_env.list_monitors()?;
                graphical_env.apply_gamma(gamma, monitors)?;
            },
        }
        Ok(())
    }
}

fn parse_time_arg(arg: &str) -> chrono::format::ParseResult<NaiveTime> {
    NaiveTime::parse_from_str(arg, "%H:%M")
}

fn create_color_channels(config: Config, time: Option<NaiveTime>) -> [f64; 3] {
    let hours = match time {
        Some(offset) => timelike_to_hours(&offset),
        None => timelike_to_hours(&Local::now()),
    };
    gamma_function(config)(hours)
}
