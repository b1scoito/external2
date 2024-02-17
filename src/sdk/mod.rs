use color_eyre::eyre::Result;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

use crate::memory::Module;

#[cfg(target_os = "linux")]
pub use linux::LinuxSdk as PlatformSdk;
#[cfg(target_os = "windows")]
pub use windows::WindowsSdk as PlatformSdk;

pub mod cs2;
pub mod entity;

// TODO: SDK Initialization does not need to be separate for each platform

pub trait Sdk {
    fn get_module(&self, name: &str) -> Option<&Module>;
    #[cfg(target_os = "linux")]
    fn get_memory(&self) -> &crate::memory::linux::LinuxMemory;
    #[cfg(target_os = "windows")]
    fn get_memory(&self) -> &crate::memory::windows::Windows;

    fn new() -> Result<Self, color_eyre::Report>
    where
        Self: Sized;
}

pub fn initialize() -> Result<Box<dyn Sdk>, color_eyre::Report> {
    Ok(Box::new(PlatformSdk::new()?))
}
