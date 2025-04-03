// fn main() {
//     println!("Hello, world!");
// }

// Below is a version of the `main` function and some error types. This assumes
// the existence of types like `FileManager`, `Packet`, and `PacketParseError`.
// You can use this code as a starting point for the exercise, or you can
// delete it and write your own code with the same function signature.

use std::{
    collections::HashMap, ffi::OsString, io::{self, Write}, net::UdpSocket, str::{self, Bytes, FromStr}
};

#[derive(Debug,PartialEq)]
pub enum Packet {
    HeaderPacket(HeaderPacket),
    DataPacket(DataPacket)
}

impl TryFrom<&[u8]> for Packet {
    type Error = PacketParseError;

    fn try_from(bytes: &[u8]) -> Result<Self, PacketParseError> {
        let status_byte: u8 = bytes[0];
        if status_byte == 0 { 
            return Ok(Packet::HeaderPacket(HeaderPacket::try_from(bytes)?));
            // let file_name = OsString::from_str(str::from_utf8(&bytes[2..bytes.len()]).unwrap()).unwrap(); // Uhhhhhh what is this line... there is probably a better way to do this?
        } else {
            return Ok(Packet::DataPacket(DataPacket::try_from(bytes)?));
        }
    }
    
}
#[derive(Debug)]
pub enum PacketParseError {
    
}

#[derive(Debug,PartialEq)]
pub struct HeaderPacket {
    status_byte: u8,
    file_id: u8,
    file_name: OsString
}

impl TryFrom<&[u8]> for HeaderPacket {
    type Error = PacketParseError;

    fn try_from(bytes: &[u8]) -> Result<Self, PacketParseError> {
        let status_byte: u8 = bytes[0];
        let file_id: u8 = bytes[1];
        let file_name: OsString = unsafe { OsString::from_encoded_bytes_unchecked(bytes[2..bytes.len()].to_vec()) };
        Ok(HeaderPacket { status_byte: status_byte, file_id: file_id, file_name: file_name })
    }
}

#[derive(Debug,PartialEq)]
pub struct  DataPacket {
    status_byte: u8,
    file_id: u8,
    packet_number: u16,
    data: Vec<u8>,
}

impl TryFrom<&[u8]> for DataPacket {
    type Error = PacketParseError;

    fn try_from(bytes: &[u8]) -> Result<Self, PacketParseError> {
        let status_byte: u8 = bytes[0];
        let file_id: u8 = bytes[1];
        let packet_number_bytes: [u8; 2] = [bytes[2], bytes[3]];
        let packet_number: u16 = u16::from_be_bytes(packet_number_bytes);
        let data: Vec<u8> = bytes[4..bytes.len()].to_vec();
        Ok(DataPacket{status_byte,file_id,packet_number,data})
    }
}

pub struct PacketGroup {
    file_name: Option<OsString>,
    file_id: u8,
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

    pub fn received_all_packets(&self) -> bool {
        let mut received: bool = false;
        for packet_group in &self.packet_groups {
            if packet_group.expected_number_of_packets == Some(packet_group.packets.len()) {
                received = true
            } else {
                received = false
            }
        }

        return received;
    }

    pub fn process_packet(&self, packet: Packet) {
        // create a new PacketGroup if there is none for the current file and puts packet in that in correct order
        // flags if it is last in packet (when it appears)
        todo!()
    }

    pub fn write_all_files(&self) {
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

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_try_into_header_packet() {
       let header_packet_bytes: [u8; 6] = [0, 1, b't', b'e', b's', b't']; // status_byte, file_id, file_name
       let packet = HeaderPacket::try_from(&header_packet_bytes[..]).unwrap();

       assert_eq!(packet, HeaderPacket{status_byte: 0, file_id: 1, file_name:  OsString::from("test")})
    }

    #[test]
    fn test_try_into_data_packet() {
       let data_packet_bytes: [u8; 6] = [1, 1, 2, 2, 3, 3]; // status_byte, file_id, file_name
       let packet = DataPacket::try_from(&data_packet_bytes[..]).unwrap();

       assert_eq!(packet, DataPacket{status_byte: 1, file_id: 1, packet_number: 514, data: vec![3,3]})
    }
}