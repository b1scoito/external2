use cheats::bhop;
use color_eyre::Result;

use sdk::entity::Entity;

use tracing_subscriber::filter::LevelFilter;

mod cheats;
mod memory;
mod sdk;

fn main() -> Result<()> {
    // Setup logging and color_eyre
    setup()?;

    // Initialize the SDK
    let sdk = sdk::initialize()?;
    let local_player = Entity::new(sdk)?;

    // Start cheats
    bhop::init(local_player)?;

    Ok(())
}

fn setup() -> Result<()> {
    // Install color_eyre as the global error handler
    color_eyre::install()?;

    // Set the log level
    let level = if cfg!(debug_assertions) {
        LevelFilter::TRACE
    } else {
        LevelFilter::TRACE
    };

    // Setup tracing subscriber
    tracing_subscriber::fmt::fmt().with_max_level(level).init();

    Ok(())
}
