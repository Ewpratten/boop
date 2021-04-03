use std::time;
use oping::{Ping};
use std::net::IpAddr;
use colored::*;

pub struct PingResult {
    is_up: bool,
    latency: time::Duration
}

pub fn ping(address: &IpAddr, timeout: time::Duration) -> Option<PingResult> {

    // Create an ICMP ping packet
    let mut packet = Ping::new();

    // Set the packet's timeout
    let _ = packet.set_timeout(timeout.as_secs_f64()).unwrap();

    // Specify the remote host
    let _ = packet.add_host(&address.to_string()).unwrap();

    // Send the ping packet
    return match packet.send() {
        Ok(mut result) => {
            match result.next() {
                Some(item) => Some(PingResult {
                    is_up: item.dropped == 0,
                    latency: time::Duration::from_millis(item.latency_ms as u64)
                }),
                None => None
            }
        },
        Err(_e) => {
            println!("{}", "Please run as ROOT".red());
            None
        }
    }

}