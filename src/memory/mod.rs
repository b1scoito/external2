/// Memory module
///
/// This module contains the [`Memory`] trait and its implementations.
/// The [`Memory`] trait is used to read and write memory from a process.
// Global imports
use color_eyre::eyre::{self, Ok, Result};

// OS-dependent imports
#[cfg(target_os = "linux")]
use nix::{
    sys::uio::{self, process_vm_readv, process_vm_writev},
    unistd::Pid,
};

#[cfg(target_os = "linux")]
use std::io::{IoSlice, IoSliceMut};

#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE},
    System::{
        Diagnostics::{
            Debug::{ReadProcessMemory, WriteProcessMemory},
            ToolHelp::{
                CreateToolhelp32Snapshot, Module32FirstW, Module32NextW, MODULEENTRY32W,
                TH32CS_SNAPMODULE,
            },
        },
        Threading::{OpenProcess, PROCESS_ALL_ACCESS},
    },
};

#[cfg(target_os = "windows")]
use sysinfo::Pid;

#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStringExt;

#[cfg(target_os = "linux")]
pub struct Linux {
    pub process_pid: Pid,
}

#[cfg(target_os = "windows")]
pub struct Windows {
    pub process_pid: Pid,
    pub process_handle: HANDLE,
}

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub base_address: usize,
    pub size: usize,
}

mod pattern;

pub trait Memory {
    fn new(process_pid: Pid) -> Result<Self>
    where
        Self: Sized;
    fn read<T>(&self, address: usize) -> Result<T>;
    fn read_into(&self, address: usize, buffer: &mut [u8]) -> Result<usize>;
    fn write<T>(&self, address: usize, value: T) -> Result<()>;
    fn get_module(&self, mod_name: &str) -> Result<Module>;
}

#[cfg(target_os = "linux")]
impl Memory for Linux {
    /// Creates a new [`LinuxMemory`].
    fn new(process_pid: Pid) -> Result<Self> {
        Ok(Self { process_pid })
    }

    /// Read memory from a process
    fn read<T>(&self, address: usize) -> Result<T> {
        let mut buffer = vec![0; std::mem::size_of::<T>()];
        let bytes_read = process_vm_readv(
            self.process_pid,
            &mut [IoSliceMut::new(&mut buffer)],
            &[uio::RemoteIoVec {
                base: address,
                len: std::mem::size_of::<T>(),
            }],
        )?;

        if bytes_read != std::mem::size_of::<T>() {
            return Err(eyre::eyre!(
                "Failed to read memory from process: {}",
                self.process_pid
            ));
        }

        Ok(unsafe { std::ptr::read(buffer.as_ptr() as *const T) })
    }

    /// Read memory from a process into a buffer
    fn read_into(&self, address: usize, buffer: &mut [u8]) -> Result<usize> {
        let buffer_len = buffer.len();
        let bytes_read = process_vm_readv(
            self.process_pid,
            &mut [IoSliceMut::new(buffer)],
            &[uio::RemoteIoVec {
                base: address,
                len: buffer_len,
            }],
        )?;

        if bytes_read != buffer_len {
            return Err(eyre::eyre!(
                "Failed to read memory from process: {}",
                self.process_pid
            ));
        }

        Ok(bytes_read)
    }

    /// Write memory to a process using process_writev
    fn write<T>(&self, address: usize, value: T) -> Result<()> {
        let buffer = unsafe {
            std::slice::from_raw_parts(&value as *const T as *const u8, std::mem::size_of::<T>())
        };
        let bytes_written = process_vm_writev(
            self.process_pid,
            &[IoSlice::new(buffer)],
            &[uio::RemoteIoVec {
                base: address,
                len: std::mem::size_of::<T>(),
            }],
        )?;

        if bytes_written != std::mem::size_of::<T>() {
            return Err(eyre::eyre!(
                "Failed to write memory to process: {}",
                self.process_pid
            ));
        }

        Ok(())
    }

    fn get_module(&self, mod_name: &str) -> Result<Module> {
        let process_maps = proc_maps::get_process_maps(self.process_pid.into())?;
        for map in process_maps {
            if map.filename().is_none() {
                continue;
            }

            match map.filename() {
                Some(filename) => {
                    if filename.to_string_lossy().contains(mod_name) && map.is_exec() {
                        return Ok(Module {
                            name: filename.to_string_lossy().to_string(),
                            base_address: map.start() as usize,
                            size: map.start() as usize,
                        });
                    }
                }
                None => continue,
            }
        }

        Err(eyre::eyre!("Module not found"))
    }
}

#[cfg(target_os = "windows")]
impl Memory for Windows {
    /// Creates a new [`Windows`].
    fn new(process_pid: Pid) -> Result<Windows> {
        let process_handle =
            unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, process_pid.as_u32())? };
        Ok(Self {
            process_pid,
            process_handle,
        })
    }

    /// Read memory from a process
    fn read<T>(&self, address: usize) -> Result<T> {
        let mut buffer = vec![0; std::mem::size_of::<T>()];
        unsafe {
            ReadProcessMemory(
                self.process_handle,
                address as *const _,
                buffer.as_mut_ptr() as *mut _,
                std::mem::size_of::<T>(),
                Some(std::ptr::null_mut()),
            )?
        };

        Ok(unsafe { std::ptr::read(buffer.as_ptr() as *const T) })
    }

    /// Read memory from a process into a buffer
    fn read_into(&self, address: usize, buffer: &mut [u8]) -> Result<usize> {
        let buffer_len = buffer.len();
        unsafe {
            ReadProcessMemory(
                self.process_handle,
                address as *const _,
                buffer.as_mut_ptr() as *mut _,
                buffer_len,
                Some(std::ptr::null_mut()),
            )?
        };

        Ok(buffer_len)
    }

    /// Write memory to a process
    fn write<T>(&self, address: usize, value: T) -> Result<()> {
        let buffer = unsafe {
            std::slice::from_raw_parts(&value as *const T as *const u8, std::mem::size_of::<T>())
        };

        unsafe {
            WriteProcessMemory(
                self.process_handle,
                address as *const _,
                buffer.as_ptr() as *mut _,
                std::mem::size_of::<T>(),
                Some(std::ptr::null_mut()),
            )?
        };

        Ok(())
    }

    fn get_module(&self, mod_name: &str) -> Result<Module> {
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, self.process_pid.as_u32())?;
            if snapshot == INVALID_HANDLE_VALUE {
                return Err(eyre::eyre!("Failed to create snapshot"));
            }

            let mut entry = MODULEENTRY32W {
                dwSize: std::mem::size_of::<MODULEENTRY32W>() as u32,
                ..Default::default()
            };

            if Module32FirstW(snapshot, &mut entry).is_ok() {
                loop {
                    let module_name = OsString::from_wide(&entry.szModule).into_string().unwrap();
                    if module_name.starts_with(mod_name) {
                        CloseHandle(snapshot)?;
                        return Ok(Module {
                            name: module_name,
                            base_address: entry.modBaseAddr as usize,
                            size: entry.modBaseSize as usize,
                        });
                    }

                    if !Module32NextW(snapshot, &mut entry).is_ok() {
                        break;
                    }
                }
            }

            CloseHandle(snapshot)?;
            Err(eyre::eyre!("Module not found"))
        }
    }
}
