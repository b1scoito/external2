use color_eyre::eyre::{self, Result};
use sysinfo::Pid;
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

use widestring::WideString;

use super::{Memory, Module};

#[derive(Debug, Clone)]
pub struct Windows {
    pub process_pid: Pid,
    pub process_handle: HANDLE,
}

impl Memory for Windows {
    /// Creates a new [`Windows`].
    fn new(process_pid: Pid) -> Result<Self> {
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
        unsafe { ReadProcessMemory(self.process_handle, address as *const _, buffer.as_mut_ptr() as *mut _, std::mem::size_of::<T>(), Some(std::ptr::null_mut()))? };

        Ok(unsafe { std::ptr::read(buffer.as_ptr() as *const T) })    
    }

    fn read_string(&self, address: usize) -> Result<String> {
        let mut buffer = Vec::new();

        for i in 0.. {
            match self.read::<u8>(address + i) {
                Ok(byte) if byte != 0 => buffer.push(byte),
                _ => break,
            }
        }

        Ok(String::from_utf8(buffer)?)   
    }

    /// Write memory to a process
    fn write<T>(&self, address: usize, value: T) -> Result<()> {
        let buffer = unsafe {
            std::slice::from_raw_parts(&value as *const T as *const u8, std::mem::size_of::<T>())
        };

        unsafe { WriteProcessMemory(self.process_handle, address as *const _, buffer.as_ptr() as *mut _, std::mem::size_of::<T>(), Some(std::ptr::null_mut()))? };

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
                    let module_name = WideString::from_vec(&entry.szModule)
                        .to_string()?
                        .split('\0') // Split by null terminator
                        .next()
                        .unwrap()
                        .to_string();
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

impl Drop for Windows {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.process_handle).unwrap();
        }
    }
}