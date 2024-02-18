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

pub mod cs2;
pub mod entity;

// TODO: SDK Initialization does not need to be separate for each platform

pub trait Sdk {
    fn get_module(&self, name: &str) -> Option<&Module>;
    #[cfg(target_os = "linux")]
    fn get_memory(&self) -> &crate::memory::linux::Linux;
    #[cfg(target_os = "windows")]
    fn get_memory(&self) -> &crate::memory::windows::Windows;

    fn new() -> Result<Self>
    where
        Self: Sized;
}

pub fn initialize() -> Result<Box<dyn Sdk>> {
    Ok(Box::new(ExternalSdk::new()?))
}
