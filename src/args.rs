use std::{io, thread, time::Duration};

use chrono::{DateTime, FixedOffset, Local};
use structopt::StructOpt;

use crate::{
    channel::{gamma_function, BLUE_CHANNEL, GREEN_CHANNEL, RED_CHANNEL},
    config::Config,
    environment::{GraphicalEnv, GraphicalEnvContext},
    hour::timelike_to_hours,
};

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
}

impl GraphicalEnvContext for DaemonSubCommand {
    type Output = ();

    fn with_graphical_env<G>(self, graphical_env: G) -> io::Result<Self::Output>
    where
        G: GraphicalEnv,
    {
        loop {
            let gamma = channels_from_time(None);
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
    time: Option<DateTime<FixedOffset>>,
}

impl GraphicalEnvContext for PrintSubCommand {
    type Output = ();

    fn with_graphical_env<G>(self, graphical_env: G) -> io::Result<Self::Output>
    where
        G: GraphicalEnv,
    {
        let gamma = channels_from_time(self.time);
        println!("{}", graphical_env.format_gamma(gamma)?);
        Ok(())
    }

    fn without_graphical_env(self) -> io::Result<Self::Output> {
        let gamma = channels_from_time(self.time);
        println!(
            "red={:.3} green={:.3} blue={:.3}",
            gamma[RED_CHANNEL], gamma[GREEN_CHANNEL], gamma[BLUE_CHANNEL],
        );
        Ok(())
    }
}

#[derive(Debug, Clone, StructOpt)]
pub struct ApplySubCommand {
    #[structopt(long = "--time")]
    #[structopt(short = "-t")]
    #[structopt(parse(try_from_str = parse_time_arg))]
    time: Option<DateTime<FixedOffset>>,
    #[structopt(long = "--monitors")]
    #[structopt(short = "-m")]
    monitors: Option<Vec<String>>,
}

impl GraphicalEnvContext for ApplySubCommand {
    type Output = ();

    fn with_graphical_env<G>(self, graphical_env: G) -> io::Result<Self::Output>
    where
        G: GraphicalEnv,
    {
        let gamma = channels_from_time(self.time);
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

fn parse_time_arg(
    arg: &str,
) -> chrono::format::ParseResult<DateTime<FixedOffset>> {
    DateTime::parse_from_str(arg, "%H:%M")
}

pub fn channels_from_time(time: Option<DateTime<FixedOffset>>) -> [f64; 3] {
    let hours = match time {
        Some(offset) => timelike_to_hours(&offset),
        None => timelike_to_hours(&Local::now()),
    };
    let config = Config::default();
    gamma_function(config)(hours)
}
