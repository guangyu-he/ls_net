pub mod linux;
pub mod mac;
pub mod route_table;

use anyhow::{Result, anyhow};

#[derive(Debug, Clone)]
pub struct RouteEntry {
    pub destination: String,
    pub gateway: String,
    pub flags: String,
    pub iface: String, // macos (Netif) 3, linux 7
    pub ip_version: IpVersion,

    #[allow(dead_code)]
    pub genmask: Option<String>, // linux 2
    pub expire: Option<String>, // macos 4
}

impl RouteEntry {
    pub fn get_field(&self, field: &str) -> Option<String> {
        match field {
            "destination" => Some(self.destination.clone()),
            "gateway" => Some(self.gateway.clone()),
            "flags" => Some(self.flags.clone()),
            "iface" => Some(self.iface.clone()),
            "genmask" => self.genmask.clone(),
            "expire" => self.expire.clone(),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IpVersion {
    IPv4,
    IPv6,
}

#[derive(Debug)]
pub struct RouteTable {
    pub ipv4_routes: Vec<RouteEntry>,
    pub ipv6_routes: Vec<RouteEntry>,
}

impl RouteTable {
    pub fn new() -> Self {
        Self {
            ipv4_routes: Vec::new(),
            ipv6_routes: Vec::new(),
        }
    }

    pub fn add_route(&mut self, route: RouteEntry) {
        match route.ip_version {
            IpVersion::IPv4 => self.ipv4_routes.push(route),
            IpVersion::IPv6 => self.ipv6_routes.push(route),
        }
    }

    pub fn get_default_gateway(&self, ip_version: IpVersion) -> Option<&RouteEntry> {
        let routes = match ip_version {
            IpVersion::IPv4 => &self.ipv4_routes,
            IpVersion::IPv6 => &self.ipv6_routes,
        };

        routes.iter().find(|route| {
            route.destination == "default"
                || route.destination == "0.0.0.0"
                || route.destination == "::/0"
        })
    }
}

fn parse_route_line(line: &str, ip_version: IpVersion) -> Result<RouteEntry> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if cfg!(target_os = "macos") {
        let destination = parts[0].to_string();
        let gateway = parts[1].to_string();
        let flags = parts[2].to_string();
        let iface = parts[3].to_string();
        let expire = {
            let last_part = parts[parts.len() - 1];
            if last_part == "Expire" {
                Some("Expire".to_string())
            } else if last_part.chars().all(|c| c.is_ascii_digit()) {
                Some(last_part.to_string())
            } else {
                None
            }
        };

        Ok(RouteEntry {
            destination,
            gateway,
            flags,
            iface,
            expire,
            ip_version,
            genmask: None,
        })
    } else if cfg!(target_os = "linux") {
        let destination = parts[0].to_string();
        let gateway = parts[1].to_string();
        let genmask = Some(parts[2].to_string());
        let flags = parts[3].to_string();
        let iface = parts[parts.len() - 1].to_string();

        Ok(RouteEntry {
            destination,
            gateway,
            genmask,
            flags,
            iface,
            expire: None,
            ip_version,
        })
    } else if cfg!(target_os = "windows") {
        todo!();
    } else {
        return Err(anyhow!("Unsupported operating system"));
    }
}
