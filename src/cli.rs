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

#[derive(Debug, Clone, StructOpt)]
pub struct ConfigArgs {
    #[structopt(long = "--min-red")]
    #[structopt(short = "-r")]
    #[structopt(default_value = "1.0")]
    min_red: f64,
    #[structopt(long = "--max-red")]
    #[structopt(short = "-R")]
    #[structopt(default_value = "1.0")]
    max_red: f64,
    #[structopt(long = "--min-green")]
    #[structopt(short = "-g")]
    #[structopt(default_value = "0.65")]
    min_green: f64,
    #[structopt(long = "--max-green")]
    #[structopt(short = "-G")]
    #[structopt(default_value = "1.0")]
    max_green: f64,
    #[structopt(long = "--min-blue")]
    #[structopt(short = "-b")]
    #[structopt(default_value = "0.45")]
    min_blue: f64,
    #[structopt(long = "--max-blue")]
    #[structopt(short = "-B")]
    #[structopt(default_value = "1.0")]
    max_blue: f64,
    #[structopt(long = "--day-start")]
    #[structopt(short = "-d")]
    #[structopt(default_value = "05:00")]
    #[structopt(parse(try_from_str = parse_time_arg))]
    day_start: NaiveTime,
    #[structopt(long = "--dusk-start")]
    #[structopt(short = "-D")]
    #[structopt(default_value = "17:00")]
    #[structopt(parse(try_from_str = parse_time_arg))]
    dusk_start: NaiveTime,
    #[structopt(long = "--night-start")]
    #[structopt(short = "-n")]
    #[structopt(default_value = "21:00")]
    #[structopt(parse(try_from_str = parse_time_arg))]
    night_start: NaiveTime,
}

impl ConfigArgs {
    pub fn create_hour_config(&self) -> Result<HourConfig, InvalidDayPhases> {
        HourConfig::default()
            .with_day_start(timelike_to_hours(&self.day_start))?
            .with_dusk_start(timelike_to_hours(&self.dusk_start))?
            .with_night_start(timelike_to_hours(&self.night_start))
    }

    pub fn create_channels_config(
        &self,
    ) -> Result<[ChannelConfig; 3], InvalidChannelBounds> {
        Ok([
            ChannelConfig::default()
                .with_min(self.min_red)?
                .with_max(self.max_red)?,
            ChannelConfig::default()
                .with_min(self.min_green)?
                .with_max(self.max_green)?,
            ChannelConfig::default()
                .with_min(self.min_blue)?
                .with_max(self.max_blue)?,
        ])
    }

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

#[derive(Debug, Clone, StructOpt)]
#[structopt(version = "0.1")]
pub struct Program {
    #[structopt(subcommand)]
    subcommand: SubCommand,
}

impl Program {
    pub fn parse() -> Self {
        Self::from_args()
    }
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

#[derive(Debug, Clone, StructOpt)]
pub enum SubCommand {
    Daemon(DaemonSubCommand),
    Print(PrintSubCommand),
    Apply(ApplySubCommand),
}

impl GraphicalEnvContext for SubCommand {
    type Output = ();

    fn with_graphical_env<G>(self, graphical_env: G) -> io::Result<Self::Output>
    where
        G: GraphicalEnv,
    {
        match self {
            Self::Daemon(subcommand) => {
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
            Self::Daemon(subcommand) => subcommand.without_graphical_env(),
            Self::Print(subcommand) => subcommand.without_graphical_env(),
            Self::Apply(subcommand) => subcommand.without_graphical_env(),
        }
    }
}

#[derive(Debug, Clone, StructOpt)]
pub struct DaemonSubCommand {
    #[structopt(long = "--sleep-seconds")]
    #[structopt(short = "-s")]
    #[structopt(default_value = "60")]
    sleep_seconds: u64,
    #[structopt(long = "--monitors")]
    #[structopt(short = "-m")]
    monitors: Option<Vec<String>>,
    #[structopt(flatten)]
    config_args: ConfigArgs,
}

impl GraphicalEnvContext for DaemonSubCommand {
    type Output = ();

    fn with_graphical_env<G>(self, graphical_env: G) -> io::Result<Self::Output>
    where
        G: GraphicalEnv,
    {
        let config = self.config_args.create_config()?;
        loop {
            let gamma = channels_from_time(config, None);
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

#[derive(Debug, Clone, StructOpt)]
pub struct PrintSubCommand {
    #[structopt(long = "--time")]
    #[structopt(short = "-t")]
    #[structopt(parse(try_from_str = parse_time_arg))]
    time: Option<NaiveTime>,
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
        let gamma = channels_from_time(config, self.time);
        println!("{}", graphical_env.format_gamma(gamma)?);
        Ok(())
    }

    fn without_graphical_env(self) -> io::Result<Self::Output> {
        let config = self.config_args.create_config()?;
        let gamma = channels_from_time(config, self.time);
        println!(
            "red={:.3} green={:.3} blue={:.3}",
            gamma[channel::RED],
            gamma[channel::GREEN],
            gamma[channel::BLUE],
        );
        Ok(())
    }
}

#[derive(Debug, Clone, StructOpt)]
pub struct ApplySubCommand {
    #[structopt(long = "--time")]
    #[structopt(short = "-t")]
    #[structopt(parse(try_from_str = parse_time_arg))]
    time: Option<NaiveTime>,
    #[structopt(long = "--monitors")]
    #[structopt(short = "-m")]
    monitors: Option<Vec<String>>,
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
        let gamma = channels_from_time(config, self.time);
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

pub fn channels_from_time(config: Config, time: Option<NaiveTime>) -> [f64; 3] {
    let hours = match time {
        Some(offset) => timelike_to_hours(&offset),
        None => timelike_to_hours(&Local::now()),
    };
    gamma_function(config)(hours)
}
