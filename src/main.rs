use std::thread;

use cheats::bhop;
use color_eyre::Result;

use sdk::{cs2::Cs2, External2Game};
use tracing_subscriber::filter::LevelFilter;

mod cheats;
mod memory;
mod sdk;

fn main() -> Result<()> {
    // Setup logging and color_eyre
    setup()?;

    // Initialize the SDK
    let platform_sdk = sdk::initialize()?;
    match platform_sdk.get_game() {
        External2Game::Cs2 => {
            log::info!("external2 detected game: Counter-Strike 2");

            let cs2_sdk = Cs2::new(platform_sdk)?;
            thread::spawn(move || {
                bhop::initialize(cs2_sdk)
            });

            // Wait for any key to be pressed
            log::warn!("press any key to exit");
            let _ = std::io::stdin().read_line(&mut String::new());
        },
    }

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
