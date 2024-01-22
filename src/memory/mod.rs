/// Memory module
///
/// This module contains the [`Memory`] trait and its implementations.
/// The [`Memory`] trait is used to read and write memory from a process.
use color_eyre::eyre::{self, Error};
use nix::{
    sys::uio::{self, process_vm_readv},
    unistd::Pid,
};
use std::io::IoSliceMut;
pub struct LinuxMemory {
    pub process_pid: Pid,
}

impl LinuxMemory {
    /// Creates a new [`LinuxMemory`].
    pub fn new(process_pid: Pid) -> Self {
        Self { process_pid }
    }

    /// Read memory from a process
    pub fn read<T>(&self, address: usize) -> Result<T, Error> {
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

    /// Write memory to a process
    pub fn write<T>(&self, address: usize, value: T) -> Result<(), Error> {
        let mut buffer = vec![0u8; std::mem::size_of::<T>()];
        let value_ptr = &value as *const T as *const u8;
        unsafe {
            std::ptr::copy_nonoverlapping(value_ptr, buffer.as_mut_ptr(), std::mem::size_of::<T>());
        }
        let bytes_written = process_vm_readv(
            self.process_pid,
            &mut [IoSliceMut::new(&mut buffer)],
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
