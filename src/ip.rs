use std::net;

pub trait Printable {
	fn print_me(&self);
}

pub struct CommonHeader {
	pub flags: u16,
	pub proto: u16,
	pub version: u8,  // upper 4 bits only
	pub ihl: u8,      // lower 4 bits only (internet header length)
	pub tot_len: u16, // total length
	pub protocol: u8, // protocol
	pub sender: net::Ipv4Addr,
	pub receiver: net::Ipv4Addr,
}

pub struct IcmpHeader {
	pub ttl: u8, // time to live
	pub id: u16,
	pub seq: u16,
	pub inv_seq: u16,
}

pub struct TcpHeader {
	pub sender_port: u16,
	pub receiver_port: u16,
}

impl TcpHeader {
	pub fn new(bytes: &[u8]) -> Self {
		Self {
			sender_port: u16::from_be_bytes([bytes[24], bytes[25]]),
			receiver_port: u16::from_be_bytes([bytes[26], bytes[27]]),
		}
	}
}

impl Printable for TcpHeader {
	fn print_me(&self) {
		println!("{} -> {}", self.sender_port, self.receiver_port);
	}
}

impl IcmpHeader {
	pub fn new(bytes: &[u8]) -> Self {
		Self {
			ttl: bytes[12],
			id: u16::from_be_bytes([bytes[28], bytes[29]]),
			seq: u16::from_be_bytes([bytes[30], bytes[31]]),
			inv_seq: u16::from_be_bytes([bytes[31], bytes[30]]),
		}
	}
}
impl Printable for IcmpHeader {
	fn print_me(&self) {
		println!(
			"id={}, seq={}/{}, ttl={}",
			self.id, self.seq, self.inv_seq, self.ttl
		)
	}
}

// converts an Assigned Internet Protocol Number into the corresponding name
pub fn aipn_to_str(code: u8) -> String {
	match code {
		1 => "ICMP".to_string(),
		6 => "TCP".to_string(),
		_ => "unknown".to_string(),
	}
}

impl CommonHeader {
	pub fn new(bytes: &[u8]) -> Self {
		Self {
			flags: u16::from_be_bytes([bytes[0], bytes[1]]),
			proto: u16::from_be_bytes([bytes[2], bytes[3]]),
			version: bytes[4] >> 4,
			ihl: bytes[4] & 0x0f,
			tot_len: u16::from_be_bytes([bytes[6], bytes[7]]),
			protocol: bytes[13],
			sender: net::Ipv4Addr::new(bytes[16], bytes[17], bytes[18], bytes[19]),
			receiver: net::Ipv4Addr::new(bytes[20], bytes[21], bytes[22], bytes[23]),
		}
	}

	pub fn print_header(&self) {
		println!(
			"Flags: {:x?} Proto: {:x?} IP: v{} IHL: {} dw",
			self.flags, self.proto, self.version, self.ihl
		);
		println!(
			"{} -> {} {} {}",
			self.sender,
			self.receiver,
			aipn_to_str(self.protocol),
			self.tot_len,
		);
	}
}

pub fn print_header<T>(common_header: &CommonHeader, specific_header: &T)
where
	T: Printable,
{
	match common_header.protocol {
		1 | 6 => {
			specific_header.print_me();
		}
		_ => {
			println!("unknown");
		}
	}
}
