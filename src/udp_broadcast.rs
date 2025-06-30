use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use std::net::{UdpSocket, SocketAddrV4, Ipv4Addr};
use std::time::Duration;

/// Get the local IP address (first non-loopback interface)
fn get_local_ip() -> Option<String> {
    let network_interfaces = NetworkInterface::show().ok()?;
    
    for itf in network_interfaces {
        if !itf.name.starts_with("lo") && !itf.name.starts_with("docker") {
            for addr in itf.addr {
                if let network_interface::Addr::V4(v4) = addr {
                    if !v4.ip.is_loopback() && !v4.ip.is_multicast() {
                        return Some(v4.ip.to_string());
                    }
                }
            }
        }
    }
    None
}

#[derive(Debug, Clone)]
pub enum AnnouncementMode {
    Periodic(u64),  // Announce every N seconds
    OnRequest,      // Only respond to discovery requests
    Limited(u64, u32), // Announce every N seconds for M times
}

/// Start UDP discovery service with configurable announcement mode
pub fn start_discovery_service(port: u16, service_name: &str, shared_key: &str, mode: AnnouncementMode) {
    match mode {
        AnnouncementMode::Periodic(interval) => {
            announce_server_periodic(port, service_name, shared_key, interval);
        }
        AnnouncementMode::OnRequest => {
            respond_to_discovery_requests(port, service_name, shared_key);
        }
        AnnouncementMode::Limited(interval, count) => {
            announce_server_limited(port, service_name, shared_key, interval, count);
        }
    }
}

/// Announce server availability with a shared key via UDP broadcast (original function).
pub fn announce_server(port: u16, service_name: &str, shared_key: &str) {
    announce_server_periodic(port, service_name, shared_key, 30);
}

/// Announce server periodically
fn announce_server_periodic(port: u16, service_name: &str, shared_key: &str, interval_secs: u64) {
    // Get the actual local IP address
    let local_ip = get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string());
    println!("Using local IP: {}", local_ip);
    
    // Create a broadcast address
    let broadcast_address = Ipv4Addr::new(255, 255, 255, 255);
    let socket = UdpSocket::bind((Ipv4Addr::new(0, 0, 0, 0), 0)).expect("Could not bind socket");
    socket.set_broadcast(true).expect("Could not set broadcast");
    socket.set_write_timeout(Some(Duration::from_secs(1))).unwrap();

    // Create the announcement message
    let announcement = format!(
        "{{\n  \"service\": \"{}\",\n  \"ip\": \"{}\",\n  \"port\": {},\n  \"key\": \"{}\"\n}}",
        service_name, local_ip, port, shared_key
    );

    loop {
        // Send the announcement
        match socket.send_to(announcement.as_bytes(), SocketAddrV4::new(broadcast_address, 8888)) {
            Ok(_) => println!("Announced server at {}:{} with key", local_ip, port),
            Err(e) => println!("Failed to send broadcast: {}", e),
        }

        // Wait before sending the next announcement
        std::thread::sleep(Duration::from_secs(interval_secs));
    }
}

/// Announce server for a limited number of times
fn announce_server_limited(port: u16, service_name: &str, shared_key: &str, interval_secs: u64, max_count: u32) {
    let local_ip = get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string());
    println!("Using local IP: {} (will announce {} times)", local_ip, max_count);
    
    let broadcast_address = Ipv4Addr::new(255, 255, 255, 255);
    let socket = UdpSocket::bind((Ipv4Addr::new(0, 0, 0, 0), 0)).expect("Could not bind socket");
    socket.set_broadcast(true).expect("Could not set broadcast");
    socket.set_write_timeout(Some(Duration::from_secs(1))).unwrap();

    let announcement = format!(
        "{{\n  \"service\": \"{}\",\n  \"ip\": \"{}\",\n  \"port\": {},\n  \"key\": \"{}\"\n}}",
        service_name, local_ip, port, shared_key
    );

    for i in 1..=max_count {
        match socket.send_to(announcement.as_bytes(), SocketAddrV4::new(broadcast_address, 8888)) {
            Ok(_) => println!("Announced server at {}:{} with key ({}/{})", local_ip, port, i, max_count),
            Err(e) => println!("Failed to send broadcast: {}", e),
        }

        if i < max_count {
            std::thread::sleep(Duration::from_secs(interval_secs));
        }
    }
    println!("Finished announcing server after {} attempts", max_count);
}

/// Respond to discovery requests only
fn respond_to_discovery_requests(port: u16, service_name: &str, shared_key: &str) {
    let local_ip = get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string());
    println!("Using local IP: {} (respond-only mode)", local_ip);
    
    let socket = UdpSocket::bind((Ipv4Addr::new(0, 0, 0, 0), 8888)).expect("Could not bind discovery socket");
    println!("Listening for discovery requests on port 8888...");
    
    let response = format!(
        "{{\n  \"service\": \"{}\",\n  \"ip\": \"{}\",\n  \"port\": {},\n  \"key\": \"{}\"\n}}",
        service_name, local_ip, port, shared_key
    );
    
    let mut buf = [0; 1024];
    
    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                let request = String::from_utf8_lossy(&buf[..amt]);
                println!("Received discovery request from {}: {}", src, request.trim());
                
                // Check if it's a discovery request
                if request.contains("DISCOVER") || request.contains("discover") {
                    match socket.send_to(response.as_bytes(), src) {
                        Ok(_) => println!("Sent response to {}", src),
                        Err(e) => println!("Failed to send response to {}: {}", src, e),
                    }
                } else {
                    println!("Ignoring non-discovery request from {}", src);
                }
            }
            Err(e) => {
                println!("Error receiving discovery request: {}", e);
            }
        }
    }
}
