use redis::{server, DEFAULT_PORT};

use clap::Parser;
use tokio::net::TcpListener;
use tokio::signal;

#[cfg(feature = "otel")]
// To be able to set the XrayPropagator
use opentelemetry::global;
#[cfg(feature = "otel")]
// To configure certain options such as sampling rate
use opentelemetry::sdk::trace as sdktrace;
#[cfg(feature = "otel")]
// For passing along the same XrayId across services
use opentelemetry_aws::trace::XrayPropagator;
#[cfg(feature = "otel")]
// The `Ext` traits are to allow the Registry to accept the
// OpenTelemetry-specific types (such as `OpenTelemetryLayer`)
use tracing_subscriber::{
    fmt, layer::SubscriberExt, util::SubscriberInitExt, util::TryInitError, EnvFilter,
};

#[tokio::main] 
pub async fn main() -> redis::Result<()> {
    set_up_logging()?;
    let cli = Cli::parse();
    let port = cli.port.unwrap_or(DEFAULT_PORT);
    let listener = TcpListener::bind(&format!("127.0.0.1:{}",port)).await?;
    server::run(listener, signal::ctrl_c()).await;
    Ok(())
}

#[derive(Parser,Debug)]
#[command(name = )]

#[cfg(not(feature = "otel"))]
fn set_up_logging() -> redis::Result<()> {
    tracing_subscriber::fmt::try_init()
}

#[cfg(feature = "otel")]
fn set_up_logging() -> Result<(),TryInitError> {
    global::set_text_map_propagator(XrayPropagator::default());
    let tracing = opentele
}



