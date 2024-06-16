use libc::{mkfifo, c_char};
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use std::os::unix::ffi::OsStrExt;
use std::io::{Error, Read, Result, Write};

use crate::message::Message;

pub struct Fifo {
    path: PathBuf
}
pub struct FifoHandle {
    read: File,
    write: File,
}
impl Fifo {
    pub fn new(path: PathBuf) -> Result<Self> {
        let os_str = path.clone().into_os_string();
        let slice = os_str.as_bytes();
        let mut bytes = Vec::with_capacity(slice.len() + 1);
        bytes.extend_from_slice(slice);
        bytes.push(0); // zero terminated string
        let _ = std::fs::remove_file(&path);
        if unsafe { mkfifo((&bytes[0]) as *const u8 as *const c_char, 0o644) } != 0 {
            Err(Error::last_os_error())
        } else {
            Ok(Fifo { path })
        }
    }
    /// Blocks until anyone connects to this fifo.
    pub fn open(&self) -> Result<FifoHandle> {
        let mut pipe = OpenOptions::new()
            .read(true)
            .open(&self.path)?;

        let mut pid_bytes = [0u8; 4];
        pipe.read_exact(&mut pid_bytes)?;
        let pid = u32::from_ne_bytes(pid_bytes);

        let read = OpenOptions::new()
            .read(true)
            .open(format!("/tmp/rust-fifo-read.{}", pid))?;

        let write = OpenOptions::new()
            .write(true)
            .open(format!("/tmp/rust-fifo-write.{}", pid))?;

        Ok(FifoHandle { read, write })
    }
}

impl Drop for Fifo {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

impl FifoHandle {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let pid = std::process::id();

        let read_fifo_path = format!("/tmp/rust-fifo-write.{}", pid);
        let read_fifo = Fifo::new(read_fifo_path.into())?;

        let write_fifo_path = format!("/tmp/rust-fifo-read.{}", pid);
        let write_fifo = Fifo::new(write_fifo_path.into())?;

        let mut pipe = OpenOptions::new()
            .write(true)
            .open(path.as_ref())?;

        let pid_bytes: [u8; 4] = u32::to_ne_bytes(pid);
        pipe.write_all(&pid_bytes)?;
        pipe.flush()?;

        let write = OpenOptions::new()
            .write(true)
            .open(&write_fifo.path)?;

        let read = OpenOptions::new()
            .read(true)
            .open(&read_fifo.path)?;

        Ok(Self { read, write })
    }

    pub fn send_message(&mut self, msg: &Message) -> Result<()> {
        let msg = bincode::serialize(msg).expect("Serialization failed");
        self.write.write_all(&usize::to_ne_bytes(msg.len()))?;
        self.write.write_all(&msg)?;
        self.write.flush()
    }

    pub fn recv_message(&mut self) -> Result<Message> {
        let mut len_bytes = [0u8; std::mem::size_of::<usize>()];
        self.read.read_exact(&mut len_bytes)?;
        let len = usize::from_ne_bytes(len_bytes);

        let mut buf = vec![0; len];
        self.read.read_exact(&mut buf)?;

        Ok(bincode::deserialize(&buf).expect("Deserialization failed"))
    }
}