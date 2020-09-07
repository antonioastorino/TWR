use std::net;
pub struct Header {
	pub flags: u16,
	pub proto: u16,
	pub version: u8,  // upper 4 bits only
	pub ihl: u8,      // lower 4 bits only (internet header length)
	pub tot_len: u16, // total length
	pub ttl: u8,      // time to live
	pub protocol: u8, // protocol
	pub sender: net::Ipv4Addr,
	pub receiver: net::Ipv4Addr,
	pub id: u16,
	pub seq: u16,
	pub inv_seq: u16,
}

// converts an Assigned Internet Protocol Number into the corresponding name
pub fn aipn_to_str(code: u8) -> String {
	match code {
		1 => "ICMP".to_string(),
		6 => "TCP".to_string(),
		_ => "unknown".to_string(),
	}
}

impl Header {
	pub fn new(bytes: &[u8]) -> Self {
		Self {
			flags: u16::from_be_bytes([bytes[0], bytes[1]]),
			proto: u16::from_be_bytes([bytes[2], bytes[3]]),
			version: bytes[4] >> 4,
			ihl: bytes[4] & 0x0f,
			tot_len: u16::from_be_bytes([bytes[6], bytes[7]]),
			ttl: bytes[12],
			protocol: bytes[13],
			sender: net::Ipv4Addr::new(bytes[16], bytes[17], bytes[18], bytes[19]),
			receiver: net::Ipv4Addr::new(bytes[20], bytes[21], bytes[22], bytes[23]),
			id: u16::from_be_bytes([bytes[28], bytes[29]]),
			seq: u16::from_be_bytes([bytes[30], bytes[31]]),
			inv_seq: u16::from_be_bytes([bytes[31], bytes[30]]),
		}
	}
}
