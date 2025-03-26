# treetrace

[![Github](https://github.com/m-lima/treetrace/actions/workflows/check.yml/badge.svg)](https://github.com/m-lima/treetrace/actions/workflows/check.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Cargo](https://img.shields.io/crates/v/treetrace.svg)](https://crates.io/crates/treetrace)
[![Documentation](https://docs.rs/treetrace/badge.svg)](https://docs.rs/treetrace)

The treetrace crate provides a tracing layer that contextualizes each event into their span
hierarchy.

When an event is printed, all the spans are printed. By default, if a new event occurs inside
the same span, just the event is printed. Otherwise, only the divergent part of the span
hierarchy will be printed.

Spans are normally not printed unless an event occurs within them. This behavior can be
changed so that they are always printed when entered by calling
[`log_spans`](builder::Builder::log_spans).

Fields of spans and events are printed inline, so that each line is a log entry. However,
setting [`multiline`](builder::Builder::multiline) prints each field in a separate line

## Examples

```rust
let layer = Builder::new(Stdout).build();
let subscriber = tracing_subscriber::registry().with(layer);
tracing::subscriber::set_global_default(subscriber).unwrap();
```
