mod ping;

use clap::{App, Arg, value_t};
use std::{net::IpAddr, time::Duration};

fn main() {
    let matches = App::new("boop")
        .author("Evan Pratten <ewpratten@gmail.com>")
        .arg(
            Arg::with_name("host")
                .takes_value(true)
                .help("Host address")
                .required(true),
        )
        .arg(
            Arg::with_name("network_mode")
                .short("n")
                .long("scan-network")
                .takes_value(false)
                .help("Scan the entire subnet of the specified host")
                .required(false),
        )
        .arg(
            Arg::with_name("timeout")
                .short("t")
                .long("timeout")
                .takes_value(true)
                .help("Specify a ping timeout in seconds")
                .required(false),
        )
        .get_matches();

    // Get data
    let host = matches.value_of("host").unwrap();
    let scan_network = matches.is_present("network_mode");
    let timeout = Duration::from_secs_f64(value_t!(matches.value_of("timeout"), f64).unwrap_or(1.0));

    // Parse out the host
    let host_ip_addr: IpAddr = match host.parse() {
        Ok(addr) => addr,
        Err(e) => panic!(e)
    };

    // We can only do a net scan with v4 addresses
    if !host_ip_addr.is_ipv4() && scan_network {
        println!("Cannot perform a subnet scan without an IPv4 address");
        return;
    }

    // Build a list of hosts to scan
    let hosts_list = Vec::new(0);

    if scan_network {

    } else {
        hosts_list.push(host_ip_addr);
    }

    // Scan all hosts in the list
    for host in hosts_list {

        // Ping the host
        let result = ping::ping(&host, timeout);

        // The printout behaviour changes based on scan vs probe modes
        if scan_network {
            
        } else {

        }
        
    }


}
