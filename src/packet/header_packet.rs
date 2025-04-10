use super::PacketParseError;

use std::ffi::OsString;

#[derive(Debug, PartialEq)]
pub struct HeaderPacket {
    pub(crate) status_byte: u8,
    pub(crate) file_id: u8,
    pub(crate) file_name: OsString,
}

impl TryFrom<&[u8]> for HeaderPacket {
    type Error = PacketParseError;

    fn try_from(bytes: &[u8]) -> Result<Self, PacketParseError> {
        let status_byte: u8 = bytes[0];
        let file_id: u8 = bytes[1];
        let file_name: OsString =
            unsafe { OsString::from_encoded_bytes_unchecked(bytes[2..bytes.len()].to_vec()) };
        Ok(HeaderPacket {
            status_byte,
            file_id,
            file_name,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use crate::packet::header_packet::HeaderPacket;

    #[test]
    fn test_try_into_header_packet() {
        let header_packet_bytes: [u8; 6] = [0, 1, b't', b'e', b's', b't'];
        let packet = HeaderPacket::try_from(&header_packet_bytes[..]).unwrap();

        assert_eq!(
            packet,
            HeaderPacket {
                status_byte: 0,
                file_id: 1,
                file_name: OsString::from("test")
            }
        );
    }
}
