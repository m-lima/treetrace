# treetrace

[![Github](https://github.com/m-lima/treetrace/actions/workflows/check.yml/badge.svg)](https://github.com/m-lima/treetrace/actions/workflows/check.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Cargo](https://img.shields.io/crates/v/treetrace.svg)](https://crates.io/crates/treetrace)
[![Documentation](https://docs.rs/treetrace/badge.svg)](https://docs.rs/treetrace)

The treetrace crate provides a tracing layer that contextualizes each event into their span
hierarchy.

## Examples

```rust
let layer = Layer::new(Stdout, false, false);
let subscriber = tracing_subscriber::registry().with(layer);
tracing::subscriber::set_global_default(subscriber).unwrap();
```

## Log output examples

### Default output

```bash
[2025-03-26 16:04:43] elo::ws::layer::serve  [5e3b] ws: f715 mode: binary
[2025-03-26 16:04:43]   INFO Version user: email@example.com latency: 3.056µs
[2025-03-26 16:05:05] elo::server::layer::logger::request  [4c2e] method: GET path: /check
[2025-03-26 16:05:05]   WARN 404 Not Found method: GET path: /check user: other@example.com latency: 243.065µs
[2025-03-26 16:05:05] elo::ws::layer::serve  [e58f] ws: b8eb mode: binary
[2025-03-26 16:05:05]   INFO Version user: other@example.com latency: 3.166µs
[2025-03-26 16:05:05]   INFO Player::Id user: other@example.com latency: 4.077µs
```

### With `log_spans`

```bash
[2025-03-26 16:21:51] app::action::token  [68a0] cluster: "my-cluster" email: "email@example.com" driver: Driver { binary: "firefox", command: "geckodriver" }
[2025-03-26 16:21:51]   app::token::new  [1ae8]
[2025-03-26 16:21:53]   app::token::login  [20ee]
[2025-03-26 16:21:54]     app::token::email  [caf3]
[2025-03-26 16:21:56]     app::token::password  [9903]
[2025-03-26 16:21:57]     app::token::totp  [04b1]
[2025-03-26 16:21:57]       app::token::write  [1c41]
[2025-03-26 16:21:59]     app::token::confirm  [159d]
[2025-03-26 16:22:02]   app::token::close  [ffb5]
[2025-03-26 16:22:39] app::action::fetch  [66e1] cluster: "my-cluster" project: Some("bla") after: None max: Some(10000) load: None
[2025-03-26 16:22:39]   app::fetcher::build  [a650]
[2025-03-26 16:22:39]     INFO Creating temp directory path: "/tmp/output.json"
[2025-03-26 16:22:39]   app::fetcher::fetch  [7cd3]
[2025-03-26 16:22:39]     app::fetcher::fetch_inner  [719e]
[2025-03-26 16:22:39]       INFO Fetching attempt 1 url: "https://download.com/all?limit=10000&project=bla"
[2025-03-26 16:22:45]       INFO Parsing response url: "https://download.com/all?limit=10000&project=bla"
[2025-03-26 16:22:45]       INFO Writing progress to temporary file file: BufWriter { writer: File, buffer: 0/8192 }
[2025-03-26 16:22:45]       INFO Renaming temporary file file: "/tmp/output.json"
[2025-03-26 16:22:45]     INFO Removed file path: "/tmp/output.json"
```
