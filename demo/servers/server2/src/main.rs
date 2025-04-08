use std::net::{TcpListener, Ipv4Addr, SocketAddrV4};
use std::io::{Read, Write};
use std::thread;

fn handle_connection(listener: TcpListener, response: &'static str) {
    let local_addr=listener.local_addr().unwrap();
    println!("Server listening on {}, with {} byte(s) response: {:?}", local_addr, response.len(), response);
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Accepted connection on {} from {}, responding: {:?}", local_addr, stream.peer_addr().unwrap(), response);
                let mut buffer = [0; 1024];
                match stream.read(&mut buffer) {
                    Ok(n) if n > 0 => {
                        if let Err(e) = stream.write_all(response.as_bytes()) {
                            println!("Failed to write response: {}", e);
                        }
                    }
                    _ => println!("Failed to read from stream"),
                }
            }
            Err(e) => println!("Connection failed: {}", e),
        }
    }
}

fn main() -> std::io::Result<()> {
    let base_port = 3001;
    let base_ip = Ipv4Addr::new(127, 0, 2, 1);
    
    let responses = vec![
        "",
        "F",
        "FF",
        "FFF",
        "FFF1",
        "FFF12",
        "\x17\x03\x03\x00\x05", // non-tls, non-http
        "\x17\x03\x03\x00\x05\x00", // same
        "\x16\x03\x03\x00\x05", // valid TLS header, so it fails with "unexpected EOF" - good.
        "\x16\x03\x03\x00\x05\x00", //same
        "HTTP/1",
        "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n",
        "HTTP/",
        "<!DOC", // emulates: $ python -m http.server -b 127.0.2.14 3001
    ];

    let handles: Vec<_> = responses.into_iter()
        .enumerate()
        .map(|(i, response)| {
            // Calculate new IP by adding to the last octet of base_ip
            let octets = base_ip.octets();
            let ip = Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3] + i as u8);
            let addr = SocketAddrV4::new(ip, base_port);
            let listener = TcpListener::bind(addr).expect("Failed to bind address");
            thread::spawn(move || {
                handle_connection(listener, response);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
