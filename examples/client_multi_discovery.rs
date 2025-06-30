use std::net::{UdpSocket, Ipv4Addr};
use serde_json::Value;
use std::time::Duration;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct ServerInfo {
    service: String,
    ip: String,
    port: u64,
    key: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Starting multi-server discovery...");
    
    let expected_key = "SECRETKEY123";
    
    // Create socket for sending discovery requests
    let socket = UdpSocket::bind((Ipv4Addr::new(0, 0, 0, 0), 0))?;
    socket.set_broadcast(true)?;
    socket.set_read_timeout(Some(Duration::from_secs(5)))?;
    
    // Send discovery request
    let discovery_message = "DISCOVER";
    let broadcast_address = (Ipv4Addr::new(255, 255, 255, 255), 8888);
    
    println!("📡 Broadcasting discovery request...");
    socket.send_to(discovery_message.as_bytes(), broadcast_address)?;
    
    // Collect all server responses
    let mut buf = [0; 1024];
    let mut servers: HashMap<String, ServerInfo> = HashMap::new();
    let mut invalid_servers = 0;
    
    println!("⏳ Waiting for server responses (5 second timeout)...");
    println!("---");
    
    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                let message = String::from_utf8_lossy(&buf[..amt]);
                
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
                                let server_id = format!("{}:{}", ip, port);
                                
                                if !servers.contains_key(&server_id) {
                                    let server_info = ServerInfo {
                                        service: service.to_string(),
                                        ip: ip.to_string(),
                                        port,
                                        key: key.to_string(),
                                    };
                                    
                                    servers.insert(server_id.clone(), server_info);
                                    
                                    println!("✅ Server discovered: {}", server_id);
                                    println!("   Service: {}", service);
                                    println!("   URL: http://{}:{}", ip, port);
                                    
                                    // Show service type with emoji
                                    let service_emoji = match service {
                                        s if s.contains("production") => "🚀",
                                        s if s.contains("staging") => "🧪", 
                                        s if s.contains("development") || s.contains("service") => "🛠️",
                                        _ => "⚙️",
                                    };
                                    println!("   Type: {} {}", service_emoji, service);
                                    println!("---");
                                } else {
                                    println!("🔄 Duplicate response from {}", server_id);
                                }
                            } else {
                                invalid_servers += 1;
                                println!("❌ Invalid key from {}: got '{}', expected '{}'", src, key, expected_key);
                            }
                        } else {
                            println!("❓ Malformed response from {}", src);
                        }
                    }
                    Err(_) => {
                        println!("❌ Non-JSON response from {}: {}", src, message.trim());
                    }
                }
            }
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut => {
                        break; // Timeout reached
                    }
                    _ => {
                        println!("❌ Network error: {}", e);
                        break;
                    }
                }
            }
        }
    }
    
    // Summary
    println!("🎯 Discovery Summary:");
    println!("   ✅ Valid servers found: {}", servers.len());
    if invalid_servers > 0 {
        println!("   ❌ Invalid/unauthorized servers: {}", invalid_servers);
    }
    println!("");
    
    if servers.is_empty() {
        println!("❌ No valid servers found. Make sure:");
        println!("   1. Servers are running with correct key: '{}'", expected_key);
        println!("   2. You're on the same network");
        println!("   3. UDP port 8888 is not blocked");
        return Ok(());
    }
    
    // Group servers by service type
    let mut service_groups: HashMap<String, Vec<&ServerInfo>> = HashMap::new();
    for server in servers.values() {
        service_groups.entry(server.service.clone())
            .or_insert_with(Vec::new)
            .push(server);
    }
    
    println!("📋 Available Services:");
    for (service_name, service_servers) in service_groups {
        println!("   🔧 {} ({} server{})", 
                 service_name, 
                 service_servers.len(),
                 if service_servers.len() == 1 { "" } else { "s" });
        
        for (i, server) in service_servers.iter().enumerate() {
            println!("      {}. http://{}:{}", i + 1, server.ip, server.port);
        }
    }
    
    println!("");
    println!("💡 Tips:");
    println!("   • Use any of the URLs above to connect to the servers");
    println!("   • Production servers are typically more stable");
    println!("   • You can connect to multiple servers simultaneously");
    
    // Health check suggestion
    if let Some(first_server) = servers.values().next() {
        println!("");
        println!("🧪 Quick health check:");
        println!("   curl http://{}:{}/health", first_server.ip, first_server.port);
    }
    
    Ok(())
}
