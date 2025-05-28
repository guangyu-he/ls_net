use crate::route_table::{IpVersion, RouteTable, parse_route_line};
use anyhow::{Result, anyhow};

/// Parses the output of the `netstat -rn` command on macOS and returns a
/// `RouteTable` containing the routes.
///
/// The function takes a string slice containing the output of the `netstat -rn`
/// command and splits it into lines. It then iterates over the lines, parsing
/// each line with `parse_route_line` and adding the resulting `RouteEntry` to
/// the `RouteTable`. The `RouteTable` is then returned.
///
/// The function skips over empty lines and sections that do not contain either
/// "Internet:" or "Internet6:", which are the headers for the IPv4 and IPv6
/// routes, respectively. It also skips over the header lines themselves.
///
/// If an error occurs while parsing a line, the function returns an error.
///
/// # Errors
///
/// If an error occurs while parsing a line, the function returns an error.
pub fn parse_macos_route_output(output: &str) -> Result<RouteTable> {
    let mut route_table = RouteTable::new();
    let lines: Vec<&str> = output.lines().collect();

    let mut current_section = None;
    let mut header_parsed = false;

    for line in lines {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        if trimmed.starts_with("Internet:") {
            current_section = Some(IpVersion::IPv4);
            header_parsed = false;
            continue;
        } else if trimmed.starts_with("Internet6:") {
            current_section = Some(IpVersion::IPv6);
            header_parsed = false;
            continue;
        }

        if !header_parsed && trimmed.starts_with("Destination") {
            header_parsed = true;
        }

        if let Some(ip_version) = &current_section {
            if header_parsed {
                if let Ok(route) = parse_route_line(trimmed, ip_version.clone()) {
                    route_table.add_route(route);
                }
            }
        }
    }

    Ok(route_table)
}

    /// Executes the `netstat -rn` command on macOS and parses its output into a
    /// `RouteTable`.
    ///
    /// The function executes the `netstat -rn` command, which prints the system's
    /// route table to stdout. It then parses the output with
    /// `parse_macos_route_output` and returns the resulting `RouteTable`.
    ///
    /// If an error occurs while executing the command or parsing the output,
    /// the function returns an error.
    ///
    /// # Errors
    ///
    /// If an error occurs while executing the command or parsing the output,
    /// the function returns an error.
pub fn get_macos_routes() -> Result<RouteTable> {
    use std::process::Command;

    let output = Command::new("netstat").args(&["-rn"]).output()?;

    if !output.status.success() {
        return Err(anyhow!("Failed to execute netstat command"));
    }

    let stdout = String::from_utf8(output.stdout)?;
    parse_macos_route_output(&stdout)
}
