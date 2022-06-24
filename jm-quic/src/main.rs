use std::thread;
use std::net::UdpSocket;

fn udp_echo(address: &str) {
    let socket = UdpSocket::bind(address).expect(&format!("could not bind socket to address {}", address));

    loop {
        let mut buf = [0u8; 1500];
        let sock = socket.try_clone().expect("failed to clone socket");
        match socket.recv_from(&mut buf) {
            Ok((_, src)) => {
                thread::spawn(move || {
                    println!("handling connection from {}", src);
                    sock.send_to(&buf, &src).expect("faile to send a response");
                });
            }
            Err(e) => {
                eprintln!("couldn't receive a datagram: {}", e)
            }
        }
    }
}

use std::collections::HashMap;

fn main() {
    let mut x = 10;
    let r = &mut x;
    *r += 10;

    let mut ports = HashMap::new();
    ports.insert("local-1", "0.0.0.0:1234".to_string());
    ports.insert("local-2", "127.0.0.1:8080".to_string());
    ports.insert("external-0", "10.0.12.34:10643".to_string());

    let mut threads = Vec::new();
    for (zone, address) in ports {
        threads.push(thread::spawn(move || {
            println!("{}: Listening on {}", zone, address);
            udp_echo(&address);
        }));
    }

    for t in threads {
        t.join().expect("failed to join listening thread");
    }
}

