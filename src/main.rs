#[allow(unused_imports)]
use std::net::UdpSocket;
use bytes::{ BufMut, BytesMut };

pub struct DNSHeader {
    id: u16,
    flags: u16,
    qd_count: u16,
    an_count: u16,
    ns_count: u16,
    ar_count: u16,
}

impl DNSHeader {
    pub fn to_bytes(&self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(12);

        buf.put_u16(self.id);
        buf.put_u16(self.flags);
        buf.put_u16(self.qd_count);
        buf.put_u16(self.an_count);
        buf.put_u16(self.ns_count);
        buf.put_u16(self.ar_count);

        buf
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

                let flags: u16 = 0b1000_0000_0000_0000;

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

                println!("{:?}", &response);

                udp_socket.send_to(response.as_ref(), source).expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
