use anyhow::{Result, anyhow};
use std::process::Command;

use crate::route_table::linux::get_linux_routes;
use crate::route_table::mac::get_macos_routes;
use crate::route_table::{IpVersion, RouteEntry, RouteTable};

/// Finds the maximum length of a given field in a vector of `RouteEntry`s.
///
/// Given a vector of `RouteEntry`s and the name of a field, this function
/// returns the maximum length of that field in all of the `RouteEntry`s. If
/// the field is not present in any of the `RouteEntry`s, it returns 0.
///
/// # Arguments
///
/// * `routes`: The vector of `RouteEntry`s to search.
/// * `field`: The name of the field to search for.
///
/// # Return
///
/// The maximum length of the field.
fn get_max_len(routes: &Vec<RouteEntry>, field: &str) -> usize {
    routes
        .iter()
        .map(|route| route.get_field(field).map(|value| value.len()).unwrap_or(0))
        .max()
        .unwrap_or(0)
}

/// Prints the system's route table to stdout.
///
/// On Windows, this function simply executes the `route print` command and
/// prints the output to stdout. On other platforms, it uses the
/// `get_macos_routes` or `get_linux_routes` functions to get the route table and
/// prints it to stdout.
///
/// The `protocol` argument can be either "ipv4", "ipv6", or "all". If "all" is
/// specified, the function prints both the IPv4 and IPv6 routes. If "ipv4" or
/// "ipv6" is specified, the function only prints the routes for that protocol.
///
/// The function prints the routes in a fixed-width format, with each field
/// aligned to the maximum length of that field in the route table.
///
/// If the function encounters an error while executing the command or getting
/// the route table, it returns an error.
///
/// # Errors
///
/// If the function encounters an error while executing the command or getting
/// the route table, it returns an error.
pub fn get_route_table(protocol: &str) -> Result<()> {
    if cfg!(target_os = "windows") {
        // TODO! parse not implemented
        return match Command::new("route").args(&["print"]).output() {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);

                    println!("Route table:\n{}", stdout);
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(anyhow!("Error executing command: {}", stderr))
                }
            }
            Err(e) => Err(anyhow!("Error executing command: {}", e)),
        };
    } else {
        let route_table: RouteTable;
        if cfg!(target_os = "macos") {
            route_table = get_macos_routes()?;
        } else if cfg!(target_os = "linux") {
            route_table = get_linux_routes()?;
        } else {
            return Err(anyhow!("Unsupported operating system"));
        }

        if protocol == "ipv4" || protocol == "all" {
            println!("\n================ IPv4 Routes ================");
            for route in &route_table.ipv4_routes {
                println!(
                    "{} {:15} {:10} {:8} {}",
                    format!(
                        "{:width$}",
                        route.destination,
                        width = get_max_len(&route_table.ipv4_routes, "destination")
                    ),
                    format!(
                        "{:width$}",
                        route.gateway,
                        width = get_max_len(&route_table.ipv4_routes, "gateway") + 2
                    ),
                    route.flags,
                    route.iface,
                    route.clone().expire.unwrap_or("".to_string())
                );
            }
            println!("================ IPv4 Default Gateway ================");
            if let Some(ipv4_gateway) = route_table.get_default_gateway(IpVersion::IPv4) {
                println!(
                    "IPv4 Default Gateway: {} via {}",
                    ipv4_gateway.gateway, ipv4_gateway.iface
                );
            }
        }

        if protocol == "ipv6" || protocol == "all" {
            println!("\n================ IPv6 Routes ================");
            for route in &route_table.ipv6_routes {
                println!(
                    "{:} {:} {:10} {:8} {:}",
                    format!(
                        "{:width$}",
                        route.destination,
                        width = get_max_len(&route_table.ipv6_routes, "destination")
                    ),
                    format!(
                        "{:width$}",
                        route.gateway,
                        width = get_max_len(&route_table.ipv6_routes, "gateway") + 2
                    ),
                    route.flags,
                    route.iface,
                    route.clone().expire.unwrap_or("".to_string())
                );
            }
            println!("================ IPv6 Default Gateway ================");
            if let Some(ipv6_gateway) = route_table.get_default_gateway(IpVersion::IPv6) {
                println!(
                    "IPv6 Default Gateway: {} via {}",
                    ipv6_gateway.gateway, ipv6_gateway.iface
                );
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_v4_route_table() {
        get_route_table("ipv4").unwrap();
    }

    #[test]
    fn test_get_v6_route_table() {
        get_route_table("ipv6").unwrap();
    }

    #[test]
    fn test_get_all_route_table() {
        get_route_table("all").unwrap();
    }
}
