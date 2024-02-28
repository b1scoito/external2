use std::{sync::Arc, thread, time::Duration};

use cheats::bhop;
use color_eyre::Result;

use sdk::{cs2::Cs2, overlay::SdkOverlay, Games};
use tracing_subscriber::filter::LevelFilter;

use crate::cheats::esp;

mod cheats;
mod memory;
mod sdk;

fn main() -> Result<()> {
    // Setup logging and color_eyre
    setup()?;

    // Initialize the SDK
    let platform_sdk = sdk::initialize()?;
    let platform_sdk_clone = Arc::clone(&platform_sdk);

    thread::spawn(move || {
        platform_sdk_clone.get_input_system().init_listen_callback()
    });
    
    match platform_sdk.get_game() {
        Games::Cs2 => {
            log::info!("external2 detected game: counter-strike 2");

            // thread-safe reference to the CS2 SDK
            let cs2_sdk = Arc::new(Cs2::new(platform_sdk)?);

            // initialize caching thread
            let cs2_sdk_clone = Arc::clone(&cs2_sdk);
            thread::spawn(move || {
                loop {
                    thread::sleep(Duration::from_secs(1));
                    cs2_sdk_clone.update_entity_cache().unwrap_or_else(|e| log::error!("cache thread error: {}", e))
                }
            });

            let cs2_sdk_clone = Arc::clone(&cs2_sdk);
            thread::spawn(move || {
                bhop::initialize(cs2_sdk_clone).unwrap_or_else(|e| log::error!("bhop error: {}", e))
            });

            let cs2_sdk_clone = Arc::clone(&cs2_sdk);
            thread::spawn(move || {
                esp::initialize(cs2_sdk_clone).unwrap_or_else(|e| log::error!("esp error: {}", e))
            });

            // Other cheats

            // Wait for any key to be pressed
            // let _ = std::io::stdin().read_line(&mut String::new());
        },
    }

    // Initialize overlay
    // thread::spawn(|| {
    egui_overlay::start(SdkOverlay::new());
    // });

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
