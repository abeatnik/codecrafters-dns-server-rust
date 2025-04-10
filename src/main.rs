#[allow(unused_imports)]
use std::io::Cursor;
use std::net::UdpSocket;
use std::env;
use bytes::{ BufMut, Bytes, BytesMut };

use codecrafters_dns_server::header::DNSHeader;
use codecrafters_dns_server::packet::DNSPacket;

fn forward_to_upstream(
    query: &DNSPacket,
    other_dns_socket: &UdpSocket,
    address: &str
) -> Result<Vec<DNSPacket>, anyhow::Error> {
    let mut packets = Vec::<DNSPacket>::new();

    for question in query.questions.iter() {
        let message = DNSPacket {
            header: DNSHeader {
                qd_count: 1,
                ..query.header.clone()
            },
            questions: vec![question.clone()],
            answers: Vec::new(),
        };

        let data = Bytes::from(message.to_bytes());
        other_dns_socket.send_to(&data, address)?;

        let mut buf = [0; 512];
        let (size, _) = other_dns_socket.recv_from(&mut buf)?;
        let mut bytes = BytesMut::from(&buf[..size]).freeze();
        
        let response = DNSPacket::from_bytes(&mut bytes)?;
        packets.push(response);
    }

    Ok(packets)
}

fn main() {
    let other_dns = {
        let args: Vec<String> = env::args().collect();
        let mut resolver_address: Option<String> = None;

        for i in 0..args.len() {
            if args[i] == "--resolver" && i + 1 < args.len() {
                resolver_address = Some(args[i + 1].clone());
                break;
            }
        }

        match resolver_address {
            Some(address) => {
                let socket = UdpSocket::bind("0.0.0.0:0").expect(
                    "Failed to bind to resolver address"
                );
                Some((socket, address))
            }
            None => None,
        }
    };

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let mut query_bytes = Bytes::copy_from_slice(&buf[..size]);

                println!("bytes received: {:?}", &query_bytes);

                let query = match DNSPacket::from_bytes(&mut query_bytes) {
                    Ok(q) => q,
                    Err(e) => {
                        eprintln!("Failed to parse DNS query: {}", e);
                        continue;
                    }
                };

                let result = match other_dns {
                    Some((ref upstream_socket, ref address)) => {
                        forward_to_upstream(&query, &upstream_socket, &address).map_err(|e| {
                            eprintln!("Error forwarding query to upstream DNS: {}", e);
                            e
                        })
                    }
                    None => { Err(anyhow::anyhow!("No upstream DNS configured")) }
                };

                let mut response = BytesMut::new();

                match result {
                    Ok(packets) => {
                        let response_pakage = DNSPacket::merge(packets);
                        response.put(response_pakage.to_bytes());
                    }
                    Err(e) => {
                        eprintln!("Error while forwarding query: {}", e);
                        continue;
                    }
                }

                udp_socket.send_to(response.as_ref(), source).expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
