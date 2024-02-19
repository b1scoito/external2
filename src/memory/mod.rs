#![allow(dead_code)]

/// Memory module
///
/// This module contains the [`Memory`] trait and its implementations.
/// The [`Memory`] trait is used to read and write memory from a process.
// Global imports
use color_eyre::eyre::Result;
use sysinfo::Pid;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "windows")]
pub mod windows;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub base_address: usize,
    pub size: usize,
}

pub trait Memory {
    fn new(process_pid: Pid) -> Result<Self>
    where
        Self: Sized;

    fn get_module(&self, mod_name: &str) -> Result<Module>;

    // Read
    fn read<T>(&self, address: usize) -> Result<T>;
    fn read_string(&self, address: usize) -> Result<String>;

    // Write
    fn write<T>(&self, address: usize, value: T) -> Result<()>;
}
