use std::{
    io,
    process::{exit, Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering::*},
        Arc,
    },
    thread,
    time::Duration,
};

use chrono::{DateTime, FixedOffset, Local};
use circadianlight::{
    channel::gamma_function,
    config::Config,
    hour::timelike_to_hours,
};
use structopt::StructOpt;

fn main() {
    if let Err(error) = Program::from_args().run() {
        eprintln!("{}", error);
        exit(-1);
    }
}

#[derive(Debug, Clone, StructOpt)]
#[structopt(version = "0.1")]
struct Program {
    #[structopt(subcommand)]
    subcommand: SubCommand,
}

impl Program {
    pub fn run(self) -> io::Result<()> {
        self.subcommand.run()
    }
}

#[derive(Debug, Clone, StructOpt)]
enum SubCommand {
    Daemon(DaemonSubCommand),
    Print(PrintSubCommand),
    Apply(ApplySubCommand),
}

impl SubCommand {
    pub fn run(self) -> io::Result<()> {
        match self {
            Self::Daemon(subcommand) => subcommand.run(),
            Self::Print(subcommand) => subcommand.run(),
            Self::Apply(subcommand) => subcommand.run(),
        }
    }
}

#[derive(Debug, Clone, StructOpt)]
struct DaemonSubCommand {
    #[structopt(long = "--sleep-seconds")]
    #[structopt(short = "-s")]
    #[structopt(default_value = "60")]
    sleep_seconds: u64,
    #[structopt(long = "--monitors")]
    #[structopt(short = "-m")]
    monitors: Option<Vec<String>>,
}

impl DaemonSubCommand {
    pub fn run(self) -> io::Result<()> {
        let terminate = Arc::new(AtomicBool::new(false));
        signal_hook::flag::register(
            signal_hook::consts::SIGINT,
            terminate.clone(),
        )?;
        signal_hook::flag::register(
            signal_hook::consts::SIGTERM,
            terminate.clone(),
        )?;

        while !terminate.load(Relaxed) {
            match &self.monitors {
                Some(monitors) => apply(monitors, None)?,
                None => apply(list_monitors()?, None)?,
            }
            thread::sleep(Duration::from_secs(self.sleep_seconds));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, StructOpt)]
struct PrintSubCommand {
    #[structopt(long = "--time")]
    #[structopt(short = "-t")]
    #[structopt(parse(try_from_str = parse_time_arg))]
    time: Option<DateTime<FixedOffset>>,
}

impl PrintSubCommand {
    pub fn run(self) -> io::Result<()> {
        println!("{}", format_gamma(self.time));
        Ok(())
    }
}

#[derive(Debug, Clone, StructOpt)]
struct ApplySubCommand {
    #[structopt(long = "--time")]
    #[structopt(short = "-t")]
    #[structopt(parse(try_from_str = parse_time_arg))]
    time: Option<DateTime<FixedOffset>>,
    #[structopt(long = "--monitors")]
    #[structopt(short = "-m")]
    monitors: Option<Vec<String>>,
}

impl ApplySubCommand {
    pub fn run(self) -> io::Result<()> {
        match self.monitors {
            Some(monitors) => apply(monitors, self.time)?,
            None => apply(list_monitors()?, self.time)?,
        }
        Ok(())
    }
}

fn parse_time_arg(
    arg: &str,
) -> chrono::format::ParseResult<DateTime<FixedOffset>> {
    DateTime::parse_from_str(arg, "%H:%M")
}

fn format_gamma(time: Option<DateTime<FixedOffset>>) -> String {
    let hours = match time {
        Some(offset) => timelike_to_hours(&offset),
        None => timelike_to_hours(&Local::now()),
    };
    let config = Config::default();
    let gamma = gamma_function(config)(hours);
    format!("{:.3}:{:.3}:{:.3}", gamma[0], gamma[1], gamma[2])
}

fn list_monitors() -> io::Result<Vec<String>> {
    let output = Command::new("xrandr")
        .arg("--listmonitors")
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .skip(1)
        .filter_map(|line| line.rsplit_once(' '))
        .map(|(_, monitor)| monitor.to_owned())
        .collect())
}

fn apply<M>(monitors: M, time: Option<DateTime<FixedOffset>>) -> io::Result<()>
where
    M: IntoIterator,
    M::Item: AsRef<str>,
{
    let gamma = format_gamma(time);
    let mut command = Command::new("xrandr");
    command.stderr(Stdio::inherit());
    for monitor in monitors {
        command
            .arg("--output")
            .arg(monitor.as_ref())
            .arg("--gamma")
            .arg(&gamma);
    }
    command.output()?;
    Ok(())
}
