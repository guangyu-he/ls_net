use anyhow::{Result, anyhow};
use std::process::Command;

use crate::route_table::linux::get_linux_routes;
use crate::route_table::mac::get_macos_routes;
use crate::route_table::{IpVersion, RouteEntry, RouteTable};

fn get_max_len(routes: &Vec<RouteEntry>, field: &str) -> usize {
    routes
        .iter()
        .map(|route| route.get_field(field).map(|value| value.len()).unwrap_or(0))
        .max()
        .unwrap_or(0)
}

/// Gets the system's route table using the `route`, `netstat`, or `ip`
/// commands, depending on the operating system.
///
/// On Windows, the `route print` command is used. On macOS, the `netstat -nr`
/// command is used. On other operating systems, the `ip route` command is used.
///
/// The output of the command is printed to the console, and the exit status of
/// the command is returned. If the command fails, an error is returned.
pub fn get_route_table(protocol: &str) -> Result<()> {
    if cfg!(target_os = "windows") {
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
