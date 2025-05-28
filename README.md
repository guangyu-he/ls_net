# ls_net

A CLI tool for displaying local network interfaces, IP addresses, and routing tables. Built in Rust, `ls_net` provides a
cross-platform way to inspect your machine's network configuration with simple commands.

## Features

- List all network interfaces and their IP addresses
- Show the main IP address of the machine
- Display the system's routing table and default gateway
- Filter interfaces by protocol: IPv4, IPv6, or both
- Colorized output for readability

## Installation

### from source

1. **Clone the repository:**
   ```sh
   git clone https://github.com/guangyu-he/ls_net
   cd ls_net
   ```
2. **Build or install with Cargo:**
   ```sh
   cargo build --release
   ```

   or

   ```sh
   cargo install --path .
   ```

### from git

```sh
cargo install --git https://github.com/guangyu-he/ls_net
```

### from crates.io

```sh
cargo install ls_net
```

## Usage

```sh
ls_net [OPTIONS]
```

### Options

- `-p`, `--protocol <PROTOCOL>`  Protocol type to use: `all`, `ipv4`, or `ipv6`. Defaults to `ipv4`.
- `-h`, `--help`                 Print help information
- `-V`, `--version`              Print version information

### Example

minimal usage

```sh
ls_net
```

for all protocols

```sh
ls_net -p all
```

## Output Example (on MacOS)

```
Local Network Interfaces and IP Addresses
Main IP address: 192.168.1.100
============================================
eth0      : 192.168.1.100
lo        : 127.0.0.1
...
============================================
Found x network interfaces (displaying x)

================ IPv4 Routes ================
Destination        Gateway             Flags      Netif    Expire
0.0.0.0            192.168.1.1         UGSC       0.0.0.0     
...
================ IPv4 Default Gateway ================
IPv4 Default Gateway: 192.168.1.1 via en0
```

## Platform Support

- **macOS** and **Linux:** Uses `netstat -nr` for route table
- **Windows:** Uses `route print`

## Dependencies

- [clap](https://crates.io/crates/clap) (argument parsing)
- [if-addrs](https://crates.io/crates/if-addrs) (network interface discovery)
- [colored](https://crates.io/crates/colored) (colorized terminal output)
- [anyhow](https://crates.io/crates/anyhow) (error handling)

## License

MIT
