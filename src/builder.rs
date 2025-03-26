use crate::{layer::Layer, output::Output};

/// A helper to build an instance of [`Layer`]
pub struct Builder<O: Output>(O);

/// A helper to build an instance of [`Layer`]
///
/// This type is only for type-safe building and not directly instatiable.
///
/// # Examples
///
/// ```
/// # use treetrace::builder::Builder;
/// # use treetrace::output::Stdout;
/// Builder::new(Stdout).multiline();
/// ```
pub struct BuilderMultiline<O: Output>(O);

/// A helper to build an instance of [`Layer`]
///
/// This type is only for type-safe building and not directly instatiable.
///
/// # Examples
///
/// ```
/// # use treetrace::builder::Builder;
/// # use treetrace::output::Stdout;
/// Builder::new(Stdout).log_spans();
/// ```
pub struct BuilderWithSpans<O: Output>(O);

/// A helper to build an instance of [`Layer`]
///
/// This type is only for type-safe building and not directly instatiable.
///
/// # Examples
///
/// ```
/// # use treetrace::builder::Builder;
/// # use treetrace::output::Stdout;
/// Builder::new(Stdout).multiline().log_spans();
/// ```
pub struct BuilderMultilineWithSpans<O: Output>(O);

impl<O: Output> Builder<O> {
    /// Creates a new builder with the given output
    pub fn new(output: O) -> Self {
        Self(output)
    }

    /// Enables printing each field in a separate line
    pub fn multiline(self) -> BuilderMultiline<O> {
        BuilderMultiline(self.0)
    }

    /// Prints each span as soon as they are entered
    pub fn log_spans(self) -> BuilderWithSpans<O> {
        BuilderWithSpans(self.0)
    }

    /// Consume this builder and instatiate a [`Layer`]
    pub fn build(self) -> Layer<O> {
        Layer::new(self.0, false, false)
    }
}

impl<O: Output> BuilderMultiline<O> {
    /// Prints each span as soon as they are entered
    pub fn log_spans(self) -> BuilderMultilineWithSpans<O> {
        BuilderMultilineWithSpans(self.0)
    }

    /// Consume this builder and instatiate a [`Layer`]
    pub fn build(self) -> Layer<O> {
        Layer::new(self.0, false, true)
    }
}

impl<O: Output> BuilderWithSpans<O> {
    /// Enables printing each field in a separate line
    pub fn multiline(self) -> BuilderMultilineWithSpans<O> {
        BuilderMultilineWithSpans(self.0)
    }

    /// Consume this builder and instatiate a [`Layer`]
    pub fn build(self) -> Layer<O> {
        Layer::new(self.0, true, false)
    }
}

impl<O: Output> BuilderMultilineWithSpans<O> {
    /// Consume this builder and instatiate a [`Layer`]
    pub fn build(self) -> Layer<O> {
        Layer::new(self.0, true, true)
    }
}
