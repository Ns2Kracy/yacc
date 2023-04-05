use tracing_subscriber::{
    fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry,
};
pub fn init_tracing_subscriber() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let formatter_layer = fmt::layer()
        .with_ansi(true)
        .with_writer(std::io::stderr)
        .without_time();

    Registry::default()
        .with(env_filter)
        .with(formatter_layer)
        .init();
}
