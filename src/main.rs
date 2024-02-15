use cheats::bhop;
use color_eyre::Result;

#[cfg(target_os = "linux")]
use sdk::{LinuxSdk, Sdk};

#[cfg(target_os = "windows")]
use sdk::{Sdk, WindowsSdk};

mod cheats;
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
    #[cfg(target_os = "linux")]
    let sdk = LinuxSdk::new()?;

    #[cfg(target_os = "windows")]
    let sdk = WindowsSdk::new()?;

    // Start cheats
    bhop::init(sdk)?;

    Ok(())
}
