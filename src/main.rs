#[allow(unused_imports)]
use std::net::UdpSocket;
use bytes::{ BufMut, BytesMut };

pub struct DNSHeader {
    id: u16,
    flags: DNSFlags,
    qd_count: u16,
    an_count: u16,
    ns_count: u16,
    ar_count: u16,
}

impl DNSHeader {
    pub fn to_bytes(&self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(12);

        buf.put_u16(self.id);
        buf.put_u16(self.flags.construct_flag_bytes());
        buf.put_u16(self.qd_count);
        buf.put_u16(self.an_count);
        buf.put_u16(self.ns_count);
        buf.put_u16(self.ar_count);

        buf
    }
}

struct DNSFlags {
    qr: bool,
    opcode: u8, //will become 4 bits later, so max is 0xF
    aa: bool,
    tc: bool,
    rd: bool,
    ra: bool,
    z: u8, //will become 3 bits later, so max is 0x7
    rcode: u8, //will become 4 bits later, so max is 0xF
}

impl DNSFlags {
    fn new_qr_only() -> Self {
        Self {
            qr: true,
            opcode: 0,
            aa: false,
            tc: false,
            rd: false,
            ra: false,
            z: 0,
            rcode: 0,
        }
    }
    fn construct_flag_bytes(&self) -> u16 {
        ((self.qr as u16) << 15) |
            (((self.opcode as u16) & 0xf) << 11) | // 11-14
            ((self.aa as u16) << 10) |
            ((self.tc as u16) << 9) |
            ((self.rd as u16) << 8) |
            ((self.ra as u16) << 7) |
            (((self.z as u16) & 0x7) << 4) | //  4-6
            ((self.rcode as u16) & 0xf) // 0-4
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let flags: DNSFlags = DNSFlags::new_qr_only();

                let num: u16 = 1234;

                let header = DNSHeader {
                    id: num,
                    flags,
                    qd_count: 0,
                    an_count: 0,
                    ns_count: 0,
                    ar_count: 0,
                };

                let response = header.to_bytes();

                udp_socket.send_to(response.as_ref(), source).expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
