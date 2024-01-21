use std::{ffi::c_void, mem::MaybeUninit};

use color_eyre::eyre::Error;

/// Memory module
///
pub struct LinuxMemory {
    pub process_pid: libc::pid_t,
}

impl LinuxMemory {
    pub fn new(process_pid: libc::pid_t) -> Self {
        Self { process_pid }
    }

    /// Read memory from a process
    pub fn read<T>(&self, address: usize) -> Result<T, Error> {
        let mut buffer = MaybeUninit::<T>::uninit();
        let buffer_ptr = buffer.as_mut_ptr() as *mut c_void;
        let bytes_read = unsafe {
            libc::ptrace(
                libc::PTRACE_PEEKDATA,
                self.process_pid,
                address as *mut c_void,
                buffer_ptr,
            )
        };

        if bytes_read == -1 {
            return Err(color_eyre::eyre::eyre!(
                "Failed to read memory from process!"
            ));
        }

        Ok(unsafe { buffer.assume_init() })
    }

    /// Write memory to a process
    pub fn write<T>(&self, address: usize, value: T) -> Result<(), Error> {
        let buffer_ptr = &value as *const T as *const c_void;
        let bytes_written = unsafe {
            libc::ptrace(
                libc::PTRACE_POKEDATA,
                self.process_pid,
                address as *mut c_void,
                buffer_ptr,
            )
        };

        if bytes_written == -1 {
            return Err(color_eyre::eyre::eyre!("Failed to write memory to process"));
        }

        Ok(())
    }
}
