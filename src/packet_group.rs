use std::collections::HashMap;

use std::ffi::OsString;

pub struct PacketGroup {
    pub(crate) file_name: Option<OsString>,
    pub(crate) file_id: u8,
    pub(crate) expected_number_of_packets: Option<usize>,
    pub(crate) packets: HashMap<u16, Vec<u8>>,
}
