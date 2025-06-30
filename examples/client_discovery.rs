use std::net::{UdpSocket, Ipv4Addr};
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting client to listen for server announcements...");
    
    // The shared key that must match the server's key
    let expected_key = "SECRETKEY123";
    
    // Bind to the broadcast port
    let socket = UdpSocket::bind((Ipv4Addr::new(0, 0, 0, 0), 8888))?;
    socket.set_broadcast(true)?;
    
    println!("Listening for broadcasts on port 8888...");
    
    let mut buf = [0; 1024];
    
    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                let message = String::from_utf8_lossy(&buf[..amt]);
                println!("Received broadcast from {}: {}", src, message);
                
                // Try to parse as JSON
                match serde_json::from_str::<Value>(&message) {
                    Ok(json) => {
                        if let (Some(service), Some(ip), Some(port), Some(key)) = (
                            json.get("service").and_then(|v| v.as_str()),
                            json.get("ip").and_then(|v| v.as_str()),
                            json.get("port").and_then(|v| v.as_u64()),
                            json.get("key").and_then(|v| v.as_str()),
                        ) {
                            if key == expected_key {
                                println!("✅ Valid server discovered!");
                                println!("   Service: {}", service);
                                println!("   IP: {}", ip);
                                println!("   Port: {}", port);
                                println!("   Key: {} (matches expected key)", key);
                                println!("   You can now connect to: http://{}:{}", ip, port);
                                println!("---");
                            } else {
                                println!("❌ Invalid key received from {}: got '{}', expected '{}'", src, key, expected_key);
                            }
                        } else {
                            println!("❓ Incomplete announcement received from {}", src);
                        }
                    }
                    Err(e) => {
                        println!("❌ Failed to parse JSON from {}: {}", src, e);
                    }
                }
            }
            Err(e) => {
                println!("Error receiving data: {}", e);
            }
        }
    }
}
