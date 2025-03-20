// fn main() {
//     println!("Hello, world!");
// }

// Below is a version of the `main` function and some error types. This assumes
// the existence of types like `FileManager`, `Packet`, and `PacketParseError`.
// You can use this code as a starting point for the exercise, or you can
// delete it and write your own code with the same function signature.

use std::{
    collections::HashMap, ffi::OsString, io::{self, Write}, net::UdpSocket
};

pub enum Packet {
    HeaderPacket(HeaderPacket),
    DataPacket(DataPacket)
}
#[derive(Debug)]
pub enum PacketParseError {
    
}

pub struct HeaderPacket {
    status_byte: u8,
    file_id: u8,
    file_name: OsString
    
}

impl HeaderPacket {
    f
}

pub struct  DataPacket {
    status_byte: u8,
    file_id: u8,
    packet_number: u16,
    data: Vec<u8>,
}


pub struct PacketGroup {
    file_name: Option<OsString>,
    expected_number_of_packets: Option<usize>,
    packets: HashMap<u16,Vec<u8>>
}

pub struct FileManager {
    packet_groups: Vec<PacketGroup>
}

impl FileManager {
    fn default() -> Self {
        let packet_groups = vec![];
        Self { packet_groups: packet_groups }
    }

    fn received_all_packets() {
        todo!()
    }

    fn process_packet() {
        todo!()
    }

    fn write_all_files() {
        todo!()
    }
}

#[derive(Debug)]
pub enum ClientError {
    IoError(std::io::Error),
    PacketParseError(PacketParseError),
}

impl From<std::io::Error> for ClientError {
    fn from(e: std::io::Error) -> Self {
        ClientError::IoError(e)
    }
}

impl From<PacketParseError> for ClientError {
    fn from(e: PacketParseError) -> Self {
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

