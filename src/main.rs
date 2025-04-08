#[allow(unused_imports)]
use std::net::UdpSocket;
use bytes::{ BufMut, BytesMut };

use codecrafters_dns_server::{ DNSHeader, DNSFlags };
use codecrafters_dns_server::DNSQuestion;

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
                let labels = vec!["codecrafters".to_string(), "io".to_string()];
                let flags: DNSFlags = DNSFlags::new(true, 0, false, false, false, false, 0, 0);
                let header = DNSHeader::new(1234, flags, 1, 0, 0, 0);
                let question = DNSQuestion::new_atype_inclass(labels);
                let mut response = BytesMut::new();
                response.put(header.to_bytes());
                response.put(question.to_bytes());
                udp_socket.send_to(response.as_ref(), source).expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
