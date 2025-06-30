# UDP Broadcast Server Discovery Usage

This document explains how to use the UDP broadcast feature to automatically discover RustLM servers on your local network.

## Overview

The RustLM server automatically broadcasts its availability via UDP every 30 seconds. Client applications can listen for these broadcasts to discover available servers without needing to know their IP addresses in advance.

## Security

The broadcast includes a shared key that both server and client must know. Only clients with the correct key will accept server announcements, preventing unauthorized connections.

## Quick Start

### 1. Start the Server

```bash
# Start the RustLM server
cargo run

# Or use the demo script
./demo_udp.sh
```

The server will output:
```
Using local IP: 192.168.1.100
Server starting on http://0.0.0.0:3000
Announced server at 192.168.1.100:3000 with key
```

### 2. Run the Client Discovery

In another terminal:

```bash
# Run the client discovery example
cargo run --example client_discovery
```

The client will output:
```
Starting client to listen for server announcements...
Listening for broadcasts on port 8888...
Received broadcast from 192.168.1.100:54321: {
  "service": "rustlm-service",
  "ip": "192.168.1.100",
  "port": 3000,
  "key": "SECRETKEY123"
}
✅ Valid server discovered!
   Service: rustlm-service
   IP: 192.168.1.100
   Port: 3000
   Key: SECRETKEY123 (matches expected key)
   You can now connect to: http://192.168.1.100:3000
---
```

## Configuration

### Changing the Shared Key

Edit the key in both server and client:

**Server** (`src/main.rs`):
```rust
let shared_key = "YOUR_SECRET_KEY_HERE";
```

**Client** (`examples/client_discovery.rs`):
```rust
let expected_key = "YOUR_SECRET_KEY_HERE";
```

### Changing the Broadcast Port

The default broadcast port is 8888. To change it:

**Server** (`src/udp_broadcast.rs`):
```rust
socket.send_to(announcement.as_bytes(), SocketAddrV4::new(broadcast_address, YOUR_PORT))
```

**Client** (`examples/client_discovery.rs`):
```rust
let socket = UdpSocket::bind((Ipv4Addr::new(0, 0, 0, 0), YOUR_PORT))?;
```

### Changing the Broadcast Interval

Edit the sleep duration in `src/udp_broadcast.rs`:
```rust
std::thread::sleep(Duration::from_secs(10)); // Broadcast every 10 seconds
```

## Broadcast Message Format

The server sends a JSON message:
```json
{
  "service": "rustlm-service",
  "ip": "192.168.1.100",
  "port": 3000,
  "key": "SECRETKEY123"
}
```

## Integration Example

Here's how to integrate UDP discovery into your own Rust client:

```rust
use std::net::{UdpSocket, Ipv4Addr};
use serde_json::Value;
use std::time::Duration;

fn discover_server(expected_key: &str, timeout_secs: u64) -> Option<(String, u16)> {
    let socket = UdpSocket::bind((Ipv4Addr::new(0, 0, 0, 0), 8888)).ok()?;
    socket.set_broadcast(true).ok()?;
    socket.set_read_timeout(Some(Duration::from_secs(timeout_secs))).ok()?;
    
    let mut buf = [0; 1024];
    
    loop {
        if let Ok((amt, _)) = socket.recv_from(&mut buf) {
            let message = String::from_utf8_lossy(&buf[..amt]);
            
            if let Ok(json) = serde_json::from_str::<Value>(&message) {
                if let (Some(ip), Some(port), Some(key)) = (
                    json.get("ip").and_then(|v| v.as_str()),
                    json.get("port").and_then(|v| v.as_u64()),
                    json.get("key").and_then(|v| v.as_str()),
                ) {
                    if key == expected_key {
                        return Some((ip.to_string(), port as u16));
                    }
                }
            }
        }
    }
}

// Usage
if let Some((server_ip, server_port)) = discover_server("SECRETKEY123", 30) {
    println!("Found server at {}:{}", server_ip, server_port);
    // Connect to the server...
}
```

## Network Requirements

- UDP port 8888 must be open for broadcast traffic
- Client and server must be on the same network segment
- Firewall must allow UDP broadcast packets

## Troubleshooting

### No Server Announcements Received

1. **Check network connectivity**: Ensure client and server are on the same network
2. **Check firewall**: UDP broadcasts might be blocked
3. **Check port availability**: Port 8888 might be in use by another application
4. **Check broadcast support**: Some network configurations don't support broadcasts

### Invalid Key Errors

1. **Check key consistency**: Ensure server and client use the exact same key
2. **Check for typos**: Keys are case-sensitive and must match exactly

### Server Not Broadcasting

1. **Check server logs**: Look for "Announced server" messages
2. **Check network interfaces**: Server might be using wrong IP address
3. **Check binding**: UDP socket binding might fail on some systems

## Advanced Usage

### Multiple Services

You can modify the service name to distinguish between different types of servers:

```rust
let service_name = "rustlm-production";  // or "rustlm-staging", etc.
```

### Custom Broadcast Data

Add more information to the broadcast message:

```rust
let announcement = format!(
    "{{\\n  \\\"service\\\": \\\"{}\\\",\\n  \\\"ip\\\": \\\"{}\\\",\\n  \\\"port\\\": {},\\n  \\\"key\\\": \\\"{}\\\",\\n  \\\"version\\\": \\\"{}\\\",\\n  \\\"capabilities\\\": [\\\"nlp\\\", \\\"sentiment\\\", \\\"summarize\\\"]\\n}}",
    service_name, local_ip, port, shared_key, "1.0.0"
);
```

### Encryption

For enhanced security, consider encrypting the broadcast message using a symmetric encryption algorithm before sending.

## Performance Considerations

- **Broadcast frequency**: More frequent broadcasts increase network traffic
- **Message size**: Keep broadcast messages small to minimize network overhead
- **Client filtering**: Implement client-side filtering to reduce processing load
