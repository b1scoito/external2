use color_eyre::eyre::{self, Result};
use nix::sys::uio::{self, process_vm_readv, process_vm_writev};
use sysinfo::Pid;

use std::io::{IoSlice, IoSliceMut};

use super::{Memory, Module};

pub struct Linux {
    pub process_pid: Pid,
}

impl Memory for Linux {
    /// Creates a new [`LinuxMemory`].
    fn new(process_pid: Pid) -> Result<Self> {
        Ok(Self { process_pid })
    }

    fn get_module(&self, mod_name: &str) -> Result<Module> {
        let process_maps = proc_maps::get_process_maps(self.process_pid.as_u32() as i32)?;

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
                            size: map.size() as usize,
                        });
                    }
                }

                None => continue,
            }
        }

        Err(eyre::eyre!("Module not found"))
    }

    fn read<T>(&self, address: usize) -> Result<T> {
        let mut buffer = vec![0; std::mem::size_of::<T>()];

        let bytes_read = process_vm_readv(
            nix::unistd::Pid::from_raw(self.process_pid.as_u32() as i32),
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

    fn write<T>(&self, address: usize, value: T) -> Result<()> {
        let buffer = unsafe {
            std::slice::from_raw_parts(&value as *const T as *const u8, std::mem::size_of::<T>())
        };

        let bytes_written = process_vm_writev(
            nix::unistd::Pid::from_raw(self.process_pid.as_u32() as i32),
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
}
