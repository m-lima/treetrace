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

/// And implementation of [`Output`] that uses any [`Write`](std::io::Write)
///
/// # Examples
///
/// ```
/// # use treetrace::output::Memory;
/// # use treetrace::builder::Builder;
/// Builder::new(Memory::new(Vec::<u8>::new()));
/// ```
#[derive(Debug)]
pub struct Memory<T>(std::sync::Mutex<T>);

impl<T: 'static + Send + std::io::Write> Memory<T> {
    /// Build a new [`Memory`] from a [`Write`](std::io::Write)
    ///
    /// # Examples
    ///
    /// ```
    /// # use treetrace::output::Memory;
    /// # use treetrace::builder::Builder;
    /// Memory::new(Vec::<u8>::new());
    /// ```
    pub fn new(buffer: T) -> Self {
        Self(std::sync::Mutex::new(buffer))
    }
}

impl<T: 'static + Send + std::io::Write> Output for Memory<T> {
    fn lock(&self) -> impl std::io::Write {
        MemoryGuard(self.0.lock().unwrap())
    }
}

struct MemoryGuard<'a, T>(std::sync::MutexGuard<'a, T>);

impl<T: 'static + Send + std::io::Write> std::io::Write for MemoryGuard<'_, T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }

    fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize> {
        self.0.write_vectored(bufs)
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.0.write_all(buf)
    }

    fn write_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> std::io::Result<()> {
        self.0.write_fmt(fmt)
    }
}
