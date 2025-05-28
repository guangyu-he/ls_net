use crate::route_table::{IpVersion, RouteTable, parse_route_line};
use anyhow::{Result, anyhow};

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

pub fn get_macos_routes() -> Result<RouteTable> {
    use std::process::Command;

    let output = Command::new("netstat").args(&["-rn"]).output()?;

    if !output.status.success() {
        return Err(anyhow!("Failed to execute netstat command"));
    }

    let stdout = String::from_utf8(output.stdout)?;
    parse_macos_route_output(&stdout)
}
