# boop
![Build](https://github.com/Ewpratten/boop/workflows/Build/badge.svg) [![crates.io](https://img.shields.io/crates/v/boop-ping)](https://crates.io/crates/boop-ping)

`boop` is a commandline tool that aims to be a super simple cross between `ping` and `nmap`. The tool can quickly probe hosts using ICMP ping packets, and report their status. This probe functionality can be mixed with the program's `-n` flag to probe an entire subnet for hosts.

## Building & Installation

To build `boop`, just use cargo:

```sh
sudo apt install liboping-dev
git clone https://github.com/ewpratten/boop.git
cd boop
cargo build
cargo install --path .
```

Or, to install from `crates.io`, use:

```sh
cargo install boop-ping
```

## Usage

```
USAGE:
    boop [FLAGS] [OPTIONS] <host>

FLAGS:
    -h, --help            Prints help information
    -n, --scan-network    Scan the entire subnet of the specified host
    -V, --version         Prints version information

OPTIONS:
    -t, --timeout <timeout>    Specify a ping timeout in seconds

ARGS:
    <host>    Host address
```

Examples:

```sh
# Pinging a host
boop 8.8.8.8

# Probing a subnet (192.168.1.0-255)
boop 192.168.1.1 -n
```