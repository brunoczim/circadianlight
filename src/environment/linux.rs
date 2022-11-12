use std::{
    io,
    process::{Command, Stdio},
};

use super::GraphicalEnv;

#[derive(Debug, Clone)]
pub struct XorgEnv {
    _priv: (),
}

impl XorgEnv {
    pub fn load() -> io::Result<Option<Self>> {
        if cfg!(target_os = "linux") {
            Ok(Some(Self { _priv: () }))
        } else {
            Ok(None)
        }
    }
}

impl GraphicalEnv for XorgEnv {
    fn list_monitors(&self) -> io::Result<Vec<String>> {
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

    fn format_gamma(&self, gamma: [f64; 3]) -> String {
        format!("{:.3}:{:.3}{:.3}", gamma[0], gamma[1], gamma[2])
    }

    fn apply_gamma<I>(&self, gamma: [f64; 3], monitors: I) -> io::Result<()>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let formatted_gamma = self.format_gamma(gamma);
        let mut command = Command::new("xrandr");
        command.stderr(Stdio::inherit());
        for monitor in monitors {
            command
                .arg("--output")
                .arg(monitor.as_ref())
                .arg("--gamma")
                .arg(&formatted_gamma);
        }
        command.output()?;
        Ok(())
    }
}
