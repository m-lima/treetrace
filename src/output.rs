/// A trait that allows locking access to a [`Write`](std::io::Write)
pub trait Output: Send + Sync + 'static {
    fn lock(&self) -> impl std::io::Write;
}

/// And implementation of [`Output`] that uses [`stdout`](std::io::Stdout)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Stdout;

impl Output for Stdout {
    fn lock(&self) -> impl std::io::Write {
        std::io::stdout().lock()
    }
}

/// And implementation of [`Output`] that uses [`stderr`](std::io::Stderr)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Stderr;

impl Output for Stderr {
    fn lock(&self) -> impl std::io::Write {
        std::io::stderr().lock()
    }
}
