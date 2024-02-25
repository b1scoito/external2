use std::sync::Arc;

use color_eyre::eyre::Result;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

use crate::memory::Module;

#[cfg(target_os = "linux")]
pub use linux::LinuxSdk as ExternalSdk;
#[cfg(target_os = "windows")]
pub use windows::WindowsSdk as ExternalSdk;

#[cfg(target_os = "linux")]
pub use crate::memory::linux::Linux as PlatformMemory;
#[cfg(target_os = "windows")]
pub use crate::memory::windows::Windows as PlatformMemory;

pub mod inputsystem;
pub mod cs2;
pub mod overlay;

// TODO: SDK Initialization does not need to be separate for each platform
#[derive(Debug, Clone, Copy)]
pub enum Games {
    Cs2,
}

impl Games {
    fn process_names() -> Vec<(&'static str, Games)> {
        vec![
            #[cfg(target_os = "linux")]
            ("cs2", Games::Cs2),
            #[cfg(target_os = "windows")]
            ("cs2.exe", Games::Cs2),
        ]
    }
}

pub trait Sdk: Send + Sync {
    fn get_module(&self, name: &str) -> Option<&Module>;
    fn get_memory(&self) -> &PlatformMemory;

    fn get_game(&self) -> Games;

    fn get_input_system(&self) -> Arc<inputsystem::InputSystem>;

    fn new() -> Result<Self>
    where
        Self: Sized;
}

pub fn initialize() -> Result<Arc<dyn Sdk>> {
    Ok(Arc::new(ExternalSdk::new()?))
}
