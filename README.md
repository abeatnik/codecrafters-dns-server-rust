[![progress-banner](https://backend.codecrafters.io/progress/dns-server/673a3fce-c8ee-4f6b-9cc2-15b3acf2d0fc)](https://app.codecrafters.io/users/abeatnik?r=2qF)

# dns-server

A basic DNS server written in Rust for the [Codecrafters DNS Server Challenge](https://app.codecrafters.io/courses/dns-server/overview).

The challenge is to implement the core parts of a DNS server from scratch: listening for queries, parsing packets, and sending valid responses, without using any DNS libraries.

## Features

- Parses incoming DNS queries over UDP
- Supports multiple questions per packet (no compression in response)
- Handles compressed label sequences in the question section
- Responds with A records (IPv4) for each query
- Response IPs are hardcoded / synthetic for now

## Running

```bash
cargo run -- 2053
dig @127.0.0.1 -p 2053 codecrafters.io
```
