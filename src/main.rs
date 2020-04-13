use chrono::prelude::*;
use chrono::Duration;
use socket2::{Socket, Domain, Type};

use std::env;
use std::io::{Read, Write};
use std::net::SocketAddr;

fn main() {
    // program <remote_ip> <src_port> <dst_port>
    // Assume the user knows what they're doing
    let args: Vec<String> = env::args().collect();
    let remote_ip = &args[1];
    let src_port: u16 = args[2].parse().unwrap();
    let dst_port: u16 = args[3].parse().unwrap();
    println!("Connecting to {}:{} from port {}", remote_ip, dst_port, src_port);

    // Prepare a socket
    let socket = Socket::new(Domain::ipv4(), Type::stream(), None).unwrap();
    let bind_str = format!("0.0.0.0:{}", src_port);
    let connect_str = format!("{}:{}", remote_ip, dst_port);
    socket.bind(&bind_str.parse::<SocketAddr>().unwrap().into()).unwrap();

    // Schedule for the next whole minute
    let now = Utc::now();
    let target = now.with_second(0).unwrap().with_nanosecond(0).unwrap()
        + Duration::minutes(1);
    println!("The time is: {}", now);
    println!("Scheduling connect for: {}", target);
    let diff = target - now;
    println!("Waiting {:.6} seconds...", diff.num_microseconds().unwrap() as f64 / 1e6);
    std::thread::sleep(diff.to_std().unwrap());
    
    // Time to go
    println!("Connecting...");
    if socket.connect(&connect_str.parse::<SocketAddr>().unwrap().into()).is_ok() {
        println!("Connected!")
    } else {
        println!("Failed to connect!");
        return;
    }
    
    // Send some data
    let mut stream = socket.into_tcp_stream();
    println!("Sending \"hello\"");
    stream.write_all("hello\n".as_bytes()).unwrap();
    let mut buf = [0u8; 1024];
    while let Ok(cnt) = stream.read(&mut buf) {
        if cnt == 0 {
            break;
        }
        println!("Received: {}", String::from_utf8_lossy(&buf[0..cnt]));
    }
    println!("Disconnected.");
}
