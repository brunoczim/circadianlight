//! Environments that depend on specific platforms.

use core::fmt;
use std::{error::Error, io};

mod linux;

#[derive(Debug, Clone)]
struct NoSupportedGraphicalEnv;

impl fmt::Display for NoSupportedGraphicalEnv {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.pad("your platform and/or environment is not supported")
    }
}

impl Error for NoSupportedGraphicalEnv {}

/// Specification for a graphical environment.
pub trait GraphicalEnv {
    /// List all currently connected monitors' name.
    fn list_monitors(&self) -> io::Result<Vec<String>>;

    /// Formats a gamma color channel array into a string that can be used to
    /// apply color correction, or that the graphical environment can
    /// understand.
    fn format_gamma(&self, gamma: [f64; 3]) -> io::Result<String>;

    /// Applies a gamma correction to screen colors.
    fn apply_gamma<I>(&self, gamma: [f64; 3], monitors: I) -> io::Result<()>
    where
        I: IntoIterator,
        I::Item: AsRef<str>;
}

impl<'this, G> GraphicalEnv for &'this G
where
    G: GraphicalEnv,
{
    fn list_monitors(&self) -> io::Result<Vec<String>> {
        (**self).list_monitors()
    }

    fn format_gamma(&self, gamma: [f64; 3]) -> io::Result<String> {
        (**self).format_gamma(gamma)
    }

    fn apply_gamma<I>(&self, gamma: [f64; 3], monitors: I) -> io::Result<()>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        (**self).apply_gamma(gamma, monitors)
    }
}

/// A context dependent on graphical environments, such that can be run with
/// any graphical environment, or even without it.
pub trait GraphicalEnvContext: Sized {
    /// Output value that the context will produce with or without a graphical
    /// environment (IF the result is Ok, of course).
    type Output;

    /// Runs the context with a given graphical environment.
    fn with_graphical_env<G>(
        self,
        graphical_env: G,
    ) -> io::Result<Self::Output>
    where
        G: GraphicalEnv;

    /// Runs the context without any graphical environment. By default, this
    /// will yield an error tellig the current environment is not supported.
    fn without_graphical_env(self) -> io::Result<Self::Output> {
        Err(io::Error::new(io::ErrorKind::Unsupported, NoSupportedGraphicalEnv))
    }
}

/// Runs the given graphical context with the OS environment, if supported,
/// otherwise runs without environment.
pub fn with_os_graphical_env<C>(context: C) -> io::Result<C::Output>
where
    C: GraphicalEnvContext,
{
    if let Some(env) = linux::XorgEnv::load()? {
        context.with_graphical_env(env)
    } else {
        context.without_graphical_env()
    }
}
