use std::path::PathBuf;
use std::io::Result;

use fifo::{Fifo, FifoHandle};
use message::Message;

pub mod fifo;
pub mod message;

pub fn listen() -> Result<()> {
    let fifo = Fifo::new(PathBuf::from("/tmp/rust-fifo"))?;
    loop {
        let mut handle = fifo.open()?;
        std::thread::spawn(move || {
            match handle.recv_message().expect("Failed to receive message") {
                Message::Print(p) => println!("{}", p),
                Message::Ack => panic!("Didn't expect Ack now."),
            }
            handle.send_message(&Message::Ack).expect("Send message failed.");
        });
    }
}

pub fn send(s: String) -> Result<()> {
    let mut handle = FifoHandle::open("/tmp/rust-fifo")?;
    handle.send_message(&Message::Print(s))?;
    match handle.recv_message()? {
        Message::Print(p) => println!("{}", p),
        Message::Ack => {},
    }
    Ok(())
}
