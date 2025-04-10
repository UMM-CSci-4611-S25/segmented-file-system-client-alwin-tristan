// Below is a version of the `main` function and some error types. This assumes
// the existence of types like `FileManager`, `Packet`, and `PacketParseError`.
// You can use this code as a starting point for the exercise, or you can
// delete it and write your own code with the same function signature.

#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![warn(clippy::perf)]
#![warn(clippy::complexity)]
#![warn(clippy::correctness)]

mod file_manager;

#[allow(unused_imports)]
use std::{
    collections::HashMap,
    ffi::OsString,
    io::{self, Write},
    net::UdpSocket,
    str::{self, Bytes, FromStr},
};

use file_manager::FileManager;
use packet::Packet;

mod packet;

mod packet_group;

#[derive(Debug)]
pub enum ClientError {
    IoError(std::io::Error),
    PacketParseError(packet::PacketParseError),
}

impl From<std::io::Error> for ClientError {
    fn from(e: std::io::Error) -> Self {
        ClientError::IoError(e)
    }
}

impl From<packet::PacketParseError> for ClientError {
    fn from(e: packet::PacketParseError) -> Self {
        Self::PacketParseError(e)
    }
}

fn main() -> Result<(), ClientError> {
    let sock = UdpSocket::bind("0.0.0.0:7077")?;

    let remote_addr = "127.0.0.1:6014";
    sock.connect(remote_addr)?;
    let mut buf = [0; 1028];

    let _ = sock.send(&buf[..1028]);

    let mut file_manager = FileManager::default();

    while !file_manager.received_all_packets() {
        let len = sock.recv(&mut buf)?;
        let packet: Packet = buf[..len].try_into()?;
        print!(".");
        io::stdout().flush()?;
        file_manager.process_packet(packet);
    }

    file_manager.write_all_files()?;

    Ok(())
}
// Don't fully delete. This is for testing purposes

// fn main() {}

// #[cfg(test)]
// mod tests {
//     use crate::{
//         file_manager::FileManager,
//         packet::{data_packet::DataPacket, header_packet::HeaderPacket, Packet},
//         *,
//     };

// }
