#![deny(warnings, clippy::pedantic)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! The treetrace crate provides a tracing layer that contextualizes each event into their span
//! hierarchy.
//!
//! When an event is printed, all the spans are printed. By default, if a new event occurs inside
//! the same span, just the event is printed. Otherwise, only the divergent part of the span
//! hierarchy will be printed.
//!
//! Spans are normally not printed unless an event occurs within them. This behavior can be
//! changed so that they are always printed when entered by calling
//! [`log_spans`](builder::Builder::log_spans).
//!
//! Fields of spans and events are printed inline, so that each line is a log entry. However,
//! setting [`multiline`](builder::Builder::multiline) prints each field in a separate line
//!
//! # Examples
//!
//! ```
//! # use treetrace::{builder::Builder, output::Stdout};
//! # use tracing_subscriber::layer::SubscriberExt;
//! let layer = Builder::new(Stdout).build();
//! let subscriber = tracing_subscriber::registry().with(layer);
//! tracing::subscriber::set_global_default(subscriber).unwrap();
//! ```

pub mod builder;
pub mod layer;
pub mod output;

pub use builder::Builder;
pub use layer::Layer;
pub use output::{Output, Stderr, Stdout};
