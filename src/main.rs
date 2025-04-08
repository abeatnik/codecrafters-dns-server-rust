#[allow(unused_imports)]
use std::net::UdpSocket;
use bytes::{ BufMut, BytesMut };

use codecrafters_dns_server::header::DNSHeader;
use codecrafters_dns_server::question::DNSQuestion;
use codecrafters_dns_server::answer::DNSAnswer;

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
                let header_bytes = &buf[..12];
                let mut header = DNSHeader::from_bytes(header_bytes);
                let labels = vec!["codecrafters".to_string(), "io".to_string()];
                header.flags.qr = true;
                header.flags.aa = false;
                header.flags.tc = false;
                header.flags.ra = false;
                header.flags.z = 0;
                header.flags.rcode = if header.flags.opcode == 0 { 0 } else { 4 };
                header.qd_count = 1;
                header.an_count = 1;
                let question = DNSQuestion::new_atype_inclass(labels.clone());
                let rdata: u32 = 0x08080808;
                let answer = DNSAnswer::new_atype_inclass(labels, 0x0001, 0x0001, 60, 4, rdata);
                let mut response = BytesMut::new();
                response.put(header.to_bytes());
                response.put(question.to_bytes());
                response.put(answer.to_bytes());
                udp_socket.send_to(response.as_ref(), source).expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
