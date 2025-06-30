use std::net::{UdpSocket, Ipv4Addr};
use serde_json::Value;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting client to request server discovery...");
    
    // The shared key that must match the server's key
    let expected_key = "SECRETKEY123";
    
    // Create socket for sending discovery requests
    let socket = UdpSocket::bind((Ipv4Addr::new(0, 0, 0, 0), 0))?;
    socket.set_broadcast(true)?;
    socket.set_read_timeout(Some(Duration::from_secs(5)))?;
    
    // Send discovery request
    let discovery_message = "DISCOVER";
    let broadcast_address = (Ipv4Addr::new(255, 255, 255, 255), 8888);
    
    println!("Sending discovery request...");
    socket.send_to(discovery_message.as_bytes(), broadcast_address)?;
    
    // Listen for responses
    let mut buf = [0; 1024];
    let mut servers_found = 0;
    
    println!("Waiting for server responses (5 second timeout)...");
    
    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                let message = String::from_utf8_lossy(&buf[..amt]);
                println!("Received response from {}: {}", src, message);
                
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
                                servers_found += 1;
                                println!("✅ Valid server #{} discovered!", servers_found);
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
                            println!("❓ Incomplete response received from {}", src);
                        }
                    }
                    Err(e) => {
                        println!("❌ Failed to parse JSON from {}: {}", src, e);
                    }
                }
            }
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut => {
                        println!("Timeout reached. Discovery complete.");
                        break;
                    }
                    _ => {
                        println!("Error receiving data: {}", e);
                        break;
                    }
                }
            }
        }
    }
    
    if servers_found == 0 {
        println!("❌ No servers found. Make sure:");
        println!("   1. A server is running with OnRequest or Periodic mode");
        println!("   2. You're on the same network");
        println!("   3. The shared key matches");
        println!("   4. UDP port 8888 is not blocked by firewall");
    } else {
        println!("🎉 Discovery complete! Found {} server(s).", servers_found);
    }
    
    Ok(())
}
