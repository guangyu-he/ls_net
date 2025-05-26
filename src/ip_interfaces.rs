use anyhow::{Result, anyhow};
use colored::Colorize;
use if_addrs::{IfAddr, Interface, get_if_addrs};

/// Gets all network interfaces and their corresponding IP addresses.
///
/// This function first gets all network interfaces and their IP addresses using
/// the `get_if_addrs` function from the `if_addrs` crate. If no interfaces are
/// found, it returns an error.
///
/// It then sorts the interfaces by name and returns them as a vector of
/// `Interface` objects.
///
/// # Errors
///
/// This function returns an error if no network interfaces are found.
fn get_ip_interfaces() -> Result<Vec<Interface>> {
    // Get all network interfaces and their corresponding IP addresses
    let interfaces = get_if_addrs()?;
    if interfaces.is_empty() {
        return Err(anyhow!("No network interfaces found."));
    }

    // Sort interfaces by name
    let mut sorted_interfaces = interfaces.clone();
    sorted_interfaces.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(sorted_interfaces)
}

/// Displays all network interfaces and their corresponding IP addresses.
///
/// This function first gets all network interfaces and their IP addresses using
/// the `get_ip_interfaces` function. If no interfaces are found, it returns an
/// error.
///
/// It then filters the interfaces by IP protocol if a protocol is specified.
/// Finally, it displays the filtered interfaces with their IP addresses in a
/// formatted string.
///
/// # Errors
///
/// This function returns an error if no network interfaces are found.
///
/// # Arguments
///
/// * `protocol`: The IP protocol to filter by. If not specified, all
///   interfaces are displayed. Supported protocols are "ipv4" and "ipv6".
pub fn display_ip_interfaces(protocol: &str) -> Result<()> {
    let sorted_interfaces = get_ip_interfaces()?;

    // Calculate the longest interface name for formatting output
    let max_name_len = sorted_interfaces
        .iter()
        .map(|interface| interface.name.len())
        .max()
        .unwrap_or(10);

    // Count displayed interfaces
    let mut displayed_count = 0;
    let len_interfaces = sorted_interfaces.len();

    // Display all interfaces and IP addresses
    for interface in sorted_interfaces {
        // Filter by protocol if specified
        match protocol {
            "ipv4" => {
                if let IfAddr::V4(_) = interface.addr {
                    // Continue if it's IPv4
                } else {
                    continue;
                }
            }
            "ipv6" => {
                if let IfAddr::V6(_) = interface.addr {
                    // Continue if it's IPv6
                } else {
                    continue;
                }
            }
            _ => (), // Show all interfaces
        }

        let ip_info = match interface.addr {
            IfAddr::V4(addr) => format!("IPv4: {}/{}", addr.ip, addr.netmask),
            IfAddr::V6(addr) => format!("IPv6: {}/{}", addr.ip, addr.netmask),
        };

        println!(
            "{}: {}",
            format!("{:width$}", interface.name, width = max_name_len)
                .blue()
                .bold(),
            ip_info.yellow()
        );

        displayed_count += 1;
    }

    println!("{}", "============================================".green());
    println!(
        "Found {} network interfaces (displaying {})",
        len_interfaces, displayed_count
    );

    Ok(())
}
