use crate::packet;

use super::packet::data_packet::DataPacket;
use super::packet::header_packet::HeaderPacket;
use super::packet::Packet;
use super::packet_group::PacketGroup;
use std::collections::HashMap;
use std::io;
use std::fs::File;
use std::io::prelude::*;

pub struct FileManager {
    pub(crate) packet_groups: Vec<PacketGroup>,
}

impl FileManager {
    pub fn default() -> Self {
        let packet_groups = vec![];
        Self { packet_groups }
    }

    pub fn received_all_packets(&self) -> bool {
        let mut received: bool = false;
        for packet_group in &self.packet_groups {
            if (packet_group.expected_number_of_packets == Some(packet_group.packets.len())) && (packet_group.file_name.is_some()) {
                received = true
            } else {
                received = false
            }
        }

        return received;
    }

    pub fn process_packet(&mut self, packet: Packet) {
        // create a new PacketGroup if there is none for the current file and puts packet in that in correct order.
        // check if a packet group has the file id of current packet and if not then create it.
        // flags if it is last in packet (when it appears)

        match packet {
            Packet::HeaderPacket(header_packet) => self.process_header_packet(header_packet),
            Packet::DataPacket(data_packet) => self.process_data_packet(data_packet),
        }
    }

    pub fn process_header_packet(&mut self, header_packet: HeaderPacket) {
        let packet_id = header_packet.file_id;

        for packet_group in &mut self.packet_groups {
            if packet_group.file_id == packet_id {
                packet_group.file_name = Some(header_packet.file_name);
                return;
            }
        }

        let packet_group = PacketGroup {
            file_name: Some(header_packet.file_name),
            file_id: packet_id,
            expected_number_of_packets: None,
            packets: HashMap::new(),
        };
        self.packet_groups.push(packet_group);
    }

    pub fn process_data_packet(&mut self, data_packet: DataPacket) {
        let packet_id = data_packet.file_id;
        let is_last_data_packet = data_packet.is_last_data_packet();

        for packet_group in &mut self.packet_groups {
            if packet_group.file_id == packet_id {
                if is_last_data_packet {
                    let expected_num_packets: Option<usize> = Some((data_packet.packet_number + 1) as usize);
                    packet_group.expected_number_of_packets = expected_num_packets;
                }
                packet_group
                    .packets
                    .insert(data_packet.packet_number, data_packet.data);
                return;
            }
        }

        let mut packets = HashMap::new();
        packets.insert(data_packet.packet_number, data_packet.data);
        let packet_group = PacketGroup {
            file_name: None,
            file_id: packet_id,
            expected_number_of_packets: Some(0),
            packets,
        };
        self.packet_groups.push(packet_group);
    }

    pub fn write_all_files(&self) -> std::io::Result<()>{
        for packet_group in &self.packet_groups {
            let mut file = File::create(packet_group.file_name.clone().unwrap())?;

            for packet in 0u16..(packet_group.packets.len()) as u16 {
                file.write_all(packet_group.packets.get(&packet).unwrap()).unwrap();
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, ffi::OsString};

    use crate::{file_manager::FileManager, packet::{data_packet::DataPacket, header_packet::HeaderPacket, Packet}, packet_group::PacketGroup};


    #[test]
    fn test_process_header_packet() {
        let packet_group1: PacketGroup = PacketGroup {
            file_name: Some(OsString::from("test")),
            file_id: 4,
            expected_number_of_packets: None,
            packets: HashMap::new(),
        };
        let mut file_manager: FileManager = FileManager {
            packet_groups: vec![packet_group1],
        };

        let header_packet_bytes: [u8; 6] = [0, 1, b't', b'e', b's', b't'];
        let packet = HeaderPacket::try_from(&header_packet_bytes[..]).unwrap();

        file_manager.process_packet(Packet::HeaderPacket(packet));

        assert_eq!(
            file_manager.packet_groups[0].file_name,
            Some(OsString::from("test"))
        );
    }

    #[test]
    fn test_empty_process_header_packet() {
        let mut file_manager: FileManager = FileManager {
            packet_groups: vec![],
        };

        let header_packet_bytes: [u8; 6] = [0, 1, b't', b'e', b's', b't'];
        let packet = HeaderPacket::try_from(&header_packet_bytes[..]).unwrap();

        assert!(file_manager.packet_groups.is_empty());
        file_manager.process_packet(Packet::HeaderPacket(packet));
        assert_eq!(file_manager.packet_groups.len(), 1);
        assert_eq!(
            file_manager.packet_groups[0].file_name,
            Some(OsString::from("test"))
        );
        assert_eq!(file_manager.packet_groups[0].file_id, 1);
    }

    #[test]
    fn test_process_data_packet() {
        let packet_group1: PacketGroup = PacketGroup {
            file_name: Some(OsString::from("test")),
            file_id: 4,
            expected_number_of_packets: None,
            packets: HashMap::new(),
        };
        let mut file_manager: FileManager = FileManager {
            packet_groups: vec![packet_group1],
        };

        let data_packet_bytes: [u8; 6] = [1, 1, 2, 2, 3, 3];
        let packet = DataPacket::try_from(&data_packet_bytes[..]).unwrap();

        file_manager.process_packet(Packet::DataPacket(packet));
        assert!(file_manager.packet_groups[1].packets.contains_key(&514));
        assert_eq!(
            file_manager.packet_groups[1].packets.get(&514),
            Some(&vec![3, 3])
        );
    }

    #[test]
    fn test_empty_process_data_packet() {
        let mut file_manager: FileManager = FileManager {
            packet_groups: vec![],
        };

        let data_packet_bytes: [u8; 6] = [1, 1, 2, 2, 3, 3];
        let packet = DataPacket::try_from(&data_packet_bytes[..]).unwrap();

        assert!(file_manager.packet_groups.is_empty());
        file_manager.process_packet(Packet::DataPacket(packet));
        assert_eq!(file_manager.packet_groups.len(), 1);
        assert!(file_manager.packet_groups[0].packets.contains_key(&514));
        assert_eq!(
            file_manager.packet_groups[0].packets.get(&514),
            Some(&vec![3, 3])
        );
    }

    #[test]
    fn test_received_all_packets(){
        let header_packet_bytes: [u8; 6] = [0, 1, b't', b'e', b's', b't'];
        let header_packet = HeaderPacket::try_from(&header_packet_bytes[..]).unwrap();
        let data_packet_bytes_1: [u8; 6] = [1, 1, 0, 1, 3, 3];
        let data_packet_1 = DataPacket::try_from(&data_packet_bytes_1[..]).unwrap();
        let data_packet_bytes_2: [u8; 6] = [11, 1, 0, 2, 3, 3];
        let data_packet_2 = DataPacket::try_from(&data_packet_bytes_2[..]).unwrap();
        
        let mut file_manager = FileManager::default();
        file_manager.process_packet(Packet::HeaderPacket(header_packet));
        file_manager.process_packet(Packet::DataPacket(data_packet_1));
        file_manager.process_packet(Packet::DataPacket(data_packet_2));

        assert!(file_manager.received_all_packets())
    }

}