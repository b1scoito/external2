use color_eyre::Result;

mod memory;
mod sdk;

fn main() -> Result<()> {
    // Install color_eyre as the global error handler
    color_eyre::install()?;

    // Setup tracing subscriber
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing_subscriber::filter::LevelFilter::DEBUG)
        .init();

    // Initialize the SDK
    sdk::init()?;

    Ok(())
}