mod ip_interfaces;
mod machine_main_ip;
mod route_table;

use anyhow::Result;
use clap::Parser;
use colored::*;

/// A CLI tool for displaying local network interfaces, IP addresses and routes.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Protocol type to use: "all", "ipv4", or "ipv6". Defaults to "ipv4".
    #[clap(short, long, default_value="ipv4", value_parser=["all","ipv4","ipv6"])]
    protocol: String,

    /// Only show the main IP address of the machine
    #[clap(long)]
    ip: bool,
}

fn run(protocol: &str, only_show_ip: bool) -> Result<()> {
    let only_show_ip = only_show_ip;

    let main_ip_addr = machine_main_ip::get_local_ip().unwrap_or_else(|e| {
        eprintln!("Error getting IP address: {}", e);
        std::process::exit(1);
    });

    if only_show_ip {
        println!("{}", main_ip_addr);
        return Ok(());
    }

    println!(
        "{}",
        "Local Network Interfaces and IP Addresses".green().bold()
    );
    match machine_main_ip::get_local_ip() {
        Ok(ip) => {
            println!(
                "{} {}",
                "Main IP address: ".blue().bold(),
                ip.to_string().yellow()
            );
        }
        Err(e) => eprintln!("Error getting IP address: {}", e),
    };

    match ip_interfaces::display_ip_interfaces(protocol) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", "============================================".red());
            eprintln!("Failed to get network interfaces: {}", e);
            println!("{}", "============================================".red());
        }
    }

    println!();

    match route_table::route_table::get_route_table(protocol) {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    run(&args.protocol, args.ip)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_v4() {
        let result = run("ipv4", false);
        assert!(result.is_ok());
    }

    #[test]
    fn run_v6() {
        let result = run("ipv6", false);
        assert!(result.is_ok());
    }

    #[test]
    fn run_all() {
        let result = run("all", false);
        assert!(result.is_ok());
    }
}
