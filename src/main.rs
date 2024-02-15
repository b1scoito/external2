use color_eyre::Result;

#[cfg(target_os = "linux")]
use sdk::{Sdk, LinuxSdk};

#[cfg(target_os = "windows")]
use sdk::{Sdk, WindowsSdk};

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
    #[cfg(target_os = "unix")]
    LinuxSdk::new().init()?;

    #[cfg(target_os = "windows")]
    WindowsSdk::new().init()?;

    Ok(())
}
