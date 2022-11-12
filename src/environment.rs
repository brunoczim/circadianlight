mod linux;

use std::io;

pub trait GraphicalEnv {
    fn list_monitors(&self) -> io::Result<Vec<String>>;

    fn format_gamma(&self, gamma: [f64; 3]) -> String;

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

    fn format_gamma(&self, gamma: [f64; 3]) -> String {
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

pub trait GraphicalEnvContext {
    type Output;

    fn with_graphical_env<G>(
        self,
        graphical_env: G,
    ) -> io::Result<Self::Output>
    where
        G: GraphicalEnv;

    fn without_graphical_env(self) -> io::Result<Self::Output>;
}

pub fn with_os_graphical_env<C, G>(context: C) -> io::Result<C::Output>
where
    C: GraphicalEnvContext,
{
    if let Some(env) = linux::XorgEnv::load()? {
        context.with_graphical_env(env)
    } else {
        context.without_graphical_env()
    }
}
