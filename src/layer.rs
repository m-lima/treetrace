use crate::output::Output;

/// A [`Layer`](tracing_subscriber::Layer) implementation that displays all spans that own an event
/// when it occurs.
///
/// When an event is printed, all the spans are printed. By default, if a new event occurs inside
/// the same span, just the event is printed. Otherwise, only the divergent part of the span
/// hierarchy will be printed.
///
/// Spans are normally not printed unless an event occurs within them. This behavior can be
/// changed so that they are always printed when entered by calling [`Builder::log_spans`].
///
/// Fields of spans and events are printed inline, so that each line is a log entry. However,
/// calling [`Builder::multiline`] prints each field in a separate line.
///
/// By default, the timestamp is printed for each entry. However, it might be useful to omit it if,
/// e.g., using `jornald`. That can be achieved by calling [`Builder::omit_timestamp`].
///
/// # Examples
///
/// ```
/// # use treetrace::{layer::Layer, output::Stdout};
/// # use tracing_subscriber::layer::SubscriberExt;
/// let layer = Layer::builder(Stdout).build();
/// let subscriber = tracing_subscriber::registry().with(layer);
/// tracing::subscriber::set_global_default(subscriber).unwrap();
/// ```
pub struct Layer<O: Output> {
    output: O,
    log_spans: bool,
    multiline: bool,
    timestamp: bool,
    last_span: std::sync::atomic::AtomicU64,
}

impl<O: Output> Layer<O> {
    /// Creates a new [`Builder`] using `output` as the [`Output`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use treetrace::{layer::Layer, output::Stdout};
    /// # use tracing_subscriber::layer::SubscriberExt;
    /// let layer = Layer::builder(Stdout).build();
    /// let subscriber = tracing_subscriber::registry().with(layer);
    /// tracing::subscriber::set_global_default(subscriber).unwrap();
    /// ```
    pub fn builder(output: O) -> Builder<O, false, false, true> {
        Builder(output)
    }
}

/// The builder for a [`Layer`].
///
/// # Examples
///
/// ```
/// # use treetrace::{layer::Layer, output::Stdout};
/// # use tracing_subscriber::layer::SubscriberExt;
/// let layer = Layer::builder(Stdout).log_spans().build();
/// let subscriber = tracing_subscriber::registry().with(layer);
/// tracing::subscriber::set_global_default(subscriber).unwrap();
/// ```
pub struct Builder<O: Output, const LOG_SPANS: bool, const MULTILINE: bool, const TIMESTAMP: bool>(
    O,
);

impl<O: Output, const MULTILINE: bool, const TIMESTAMP: bool>
    Builder<O, false, MULTILINE, TIMESTAMP>
{
    /// Log spans on every entry.
    ///
    /// If disabled, spans are only loged if an event occurs.
    pub fn log_spans(self) -> Builder<O, true, MULTILINE, TIMESTAMP> {
        Builder(self.0)
    }
}

impl<O: Output, const LOG_SPANS: bool, const TIMESTAMP: bool>
    Builder<O, LOG_SPANS, false, TIMESTAMP>
{
    /// Split the log fields into multiple lines.
    ///
    /// This option can enable log forging by a malicious source.
    pub fn multiline(self) -> Builder<O, LOG_SPANS, true, TIMESTAMP> {
        Builder(self.0)
    }
}

impl<O: Output, const LOG_SPANS: bool, const MULTILINE: bool>
    Builder<O, LOG_SPANS, MULTILINE, true>
{
    /// Omit the timestamp in the log.
    ///
    /// Useful when using with, e.g., `journald` to avoid noise.
    pub fn omit_timestamp(self) -> Builder<O, LOG_SPANS, MULTILINE, false> {
        Builder(self.0)
    }
}

impl<O: Output, const LOG_SPANS: bool, const MULTILINE: bool, const TIMESTAMP: bool>
    Builder<O, LOG_SPANS, MULTILINE, TIMESTAMP>
{
    /// Constructs the final [`Layer`] instance with the options provided.
    pub fn build(self) -> Layer<O> {
        Layer {
            output: self.0,
            log_spans: LOG_SPANS,
            multiline: MULTILINE,
            timestamp: TIMESTAMP,
            last_span: std::sync::atomic::AtomicU64::new(0),
        }
    }
}

struct SpanInfo {
    id: u16,
    date_time: Option<chrono::DateTime<chrono::Utc>>,
    records: Vec<(&'static str, String)>,
    new: std::sync::atomic::AtomicBool,
}

impl SpanInfo {
    fn new(attrs: &tracing::span::Attributes<'_>, timestamp: bool) -> Self {
        use rand::SeedableRng;

        struct Visistor(Vec<(&'static str, String)>);

        impl tracing_subscriber::field::Visit for Visistor {
            fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
                self.0.push((field.name(), format!("{value:?}")));
            }
        }

        let mut visitor = Visistor(Vec::with_capacity(attrs.fields().len()));
        attrs.record(&mut visitor);

        Self {
            id: rand::Rng::random(&mut rand::rngs::SmallRng::from_os_rng()),
            date_time: timestamp.then_some(chrono::Utc::now()),
            records: visitor.0,
            new: std::sync::atomic::AtomicBool::new(true),
        }
    }
}

impl<O: Output> tracing_subscriber::Layer<tracing_subscriber::Registry> for Layer<O> {
    fn on_new_span(
        &self,
        attrs: &tracing::span::Attributes<'_>,
        id: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, tracing_subscriber::Registry>,
    ) {
        if let Some(span) = ctx.span(id) {
            if span.extensions().get::<SpanInfo>().is_none() {
                span.extensions_mut()
                    .insert(SpanInfo::new(attrs, self.timestamp));
            }

            if self.log_spans {
                let mut stdout = self.output.lock();

                let depth = ctx.span_scope(id).map_or(0, std::iter::Iterator::count);
                let last_span = self.last_span.load(std::sync::atomic::Ordering::Relaxed);

                print_span(
                    &mut stdout,
                    last_span,
                    depth.max(1) - 1,
                    Some(span).as_ref(),
                    self.multiline,
                );

                self.last_span
                    .store(id.into_u64(), std::sync::atomic::Ordering::Relaxed);
            }
        }
    }

    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        ctx: tracing_subscriber::layer::Context<'_, tracing_subscriber::Registry>,
    ) {
        let mut stdout = self.output.lock();

        let depth = ctx.event_scope(event).map_or(0, std::iter::Iterator::count);
        let current_span = ctx.current_span().id().and_then(|id| ctx.span(id));
        let last_span = self.last_span.load(std::sync::atomic::Ordering::Relaxed);

        print_span(
            &mut stdout,
            last_span,
            depth.max(1) - 1,
            current_span.as_ref(),
            self.multiline,
        );

        self.last_span.store(
            current_span.as_ref().map_or(0, |s| s.id().into_u64()),
            std::sync::atomic::Ordering::Relaxed,
        );

        print_event(&mut stdout, event, depth, self.multiline, self.timestamp);
    }

    fn on_close(
        &self,
        id: tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, tracing_subscriber::Registry>,
    ) {
        let lock = self.output.lock();
        let last_span = self.last_span.load(std::sync::atomic::Ordering::Relaxed);

        if last_span == id.into_u64() {
            let prev_span = ctx
                .span(&id)
                .and_then(|s| s.parent())
                .map_or(0, |p| p.id().into_u64());
            self.last_span
                .store(prev_span, std::sync::atomic::Ordering::Relaxed);
        }
        drop(lock);
    }
}

fn print_span(
    out: &mut impl std::io::Write,
    last_span: u64,
    depth: usize,
    span: Option<&tracing_subscriber::registry::SpanRef<'_, tracing_subscriber::Registry>>,
    multiline: bool,
) {
    if let Some(span) = span {
        if let Some(info) = span.extensions().get::<SpanInfo>() {
            let new = info.new.swap(false, std::sync::atomic::Ordering::Relaxed);

            if span.id().into_u64() != last_span || new {
                print_span(
                    out,
                    last_span,
                    depth.max(1) - 1,
                    span.parent().as_ref(),
                    multiline,
                );

                let path = span.metadata().target();
                let name = span.name();
                let div = if path.is_empty() || name.is_empty() {
                    ""
                } else {
                    "::"
                };

                if let Some(date_time) = info.date_time {
                    drop(write!(
                        out,
                        "[;2m[{timestamp}] ",
                        timestamp = date_time.format("%Y-%m-%d %H:%M:%S"),
                    ));
                }

                drop(write!(
                    out,
                    "[m{indent:>0$}[m{path}{div}[37m{name}",
                    depth * 2,
                    indent = "",
                ));

                for (k, v) in &info.records {
                    if *k == "message" {
                        let space = if path.is_empty() && name.is_empty() {
                            ""
                        } else {
                            " "
                        };
                        drop(write!(out, "[m{space}{v}"));
                        break;
                    }
                }

                drop(write!(
                    out,
                    "{arrow} [37m[{id:04x}][36m",
                    arrow = if new { " " } else { "[93m^" },
                    id = info.id,
                ));

                for (k, v) in &info.records {
                    if *k == "message" {
                        continue;
                    }

                    if multiline {
                        drop(write!(
                            out,
                            "\n{indent:>0$}- [2m{k}: [22m{v}",
                            depth * 2 + 22,
                            indent = ""
                        ));
                    } else {
                        drop(write!(out, " [2m{k}: [22m{v}"));
                    }
                }
                drop(writeln!(out, "[m"));
            }
        } else {
            drop(writeln!(out, "[31mFailed to read span info[m"));
        }
    }
}

fn print_event(
    out: &mut impl std::io::Write,
    event: &tracing::Event<'_>,
    depth: usize,
    multiline: bool,
    timestamp: bool,
) {
    struct Messenger<'w, W>(&'w mut W);
    impl<W: std::io::Write> tracing_subscriber::field::Visit for Messenger<'_, W> {
        fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
            if field.name() == "message" {
                drop(write!(self.0, " {value:?}"));
            }
        }
    }

    struct Fielder<'w, W>(&'w mut W, Option<usize>);
    impl<W: std::io::Write> tracing_subscriber::field::Visit for Fielder<'_, W> {
        fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
            if field.name() != "message" {
                if let Some(depth) = self.1 {
                    let gap = depth + 22;
                    drop(write!(
                        self.0,
                        "\n{indent:>gap$}- [36;2m{field}: [22m{value:?}",
                        indent = ""
                    ));
                } else {
                    drop(write!(self.0, " [36;2m{field}: [22m{value:?}"));
                }
            }
        }
    }

    if timestamp {
        drop(write!(
            out,
            "[;2m[{timestamp}] ",
            timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
        ));
    }

    let depth = depth * 2;
    drop(write!(
        out,
        "[m{indent:>depth$}{level}[m",
        indent = "",
        level = match *event.metadata().level() {
            tracing::Level::TRACE => {
                "[94mTRACE"
            }
            tracing::Level::DEBUG => {
                "[34mDEBUG"
            }
            tracing::Level::INFO => {
                "[32mINFO"
            }
            tracing::Level::WARN => {
                "[33mWARN"
            }
            tracing::Level::ERROR => {
                "[31mERROR"
            }
        }
    ));

    event.record(&mut Messenger(out));
    event.record(&mut Fielder(out, multiline.then_some(depth)));
    drop(writeln!(out, "[m"));
}
