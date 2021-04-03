mod ping;

use clap::{value_t, App, Arg};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use nix;
use std::{
    net::{IpAddr, Ipv4Addr},
    time::Duration,
};
use sudo;

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
    let timeout =
        Duration::from_secs_f64(value_t!(matches.value_of("timeout"), f64).unwrap_or(1.0));

    // Ensure the user is root (raw sockets cannot be opened by regular users)
    if !nix::unistd::getuid().is_root() {
        let _ = sudo::escalate_if_needed().unwrap();
    }

    // Parse out the host
    let host_ip_addr: IpAddr = match host.parse() {
        Ok(addr) => addr,
        Err(e) => panic!("{}", e),
    };

    // We can only do a net scan with v4 addresses
    if !host_ip_addr.is_ipv4() && scan_network {
        println!("Cannot perform a subnet scan without an IPv4 address");
        return;
    }

    // Set up a progress bar. This is only really used in network scan mode
    let progress_bar = ProgressBar::new(255);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{wide_bar:.blue/cyan}] [{eta} remaining]")
            .progress_chars("#>-"),
    );

    // Build a list of hosts to scan
    let mut hosts_list: Vec<IpAddr> = Vec::new();

    if scan_network {
        // Split the host into chunks
        let host_chunks: Vec<_> = host.split(".").collect();

        // Get the first three components of the address
        let components = [
            host_chunks[0].parse().unwrap(),
            host_chunks[1].parse().unwrap(),
            host_chunks[2].parse().unwrap(),
        ];

        // Push every host possible
        for i in 1..255 {
            hosts_list.push(Ipv4Addr::new(components[0], components[1], components[2], i).into());
        }

        println!(
            "Scanning address range {}",
            format!(
                "{}.{}.{}.1-255",
                components[0], components[1], components[2]
            )
            .blue()
        )
    } else {
        hosts_list.push(host_ip_addr);
    }

    // Scan all hosts in the list
    for host in hosts_list {
        // Ping the host
        let result = ping::ping(&host, timeout);

        // The printout behavior changes based on scan vs probe modes
        if scan_network {
            // Handle logging the host status
            if result.is_some() {
                let result = result.unwrap();
                if result.is_up {
                    // Print the status
                    progress_bar.println(&format!(
                        "Host {} is up ({}ms)",
                        host.to_string().blue(),
                        result.latency.as_millis()
                    ));
                }
            }

            // Update the progress bar
            progress_bar.inc(1);
        } else {
            if result.is_some() {
                let result = result.unwrap();
                if result.is_up {
                    println!(
                        "Host {} is up ({}ms)",
                        host.to_string().blue(),
                        result.latency.as_millis()
                    );
                } else {
                    println!("Host {} is down", host.to_string().blue());
                }
            } else {
                println!("Host {} is down", host.to_string().blue());
            }
        }
    }

    // Finish the progress bar
    progress_bar.finish_and_clear()
}
