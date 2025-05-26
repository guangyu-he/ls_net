use anyhow::{Result, anyhow};
use std::net::{SocketAddr, UdpSocket};

/// Gets the local machine's main IPv4 address.
///
/// This function creates a UDP socket, binds it to any available address and
/// port, and then connects to Google's public DNS server. It then returns the
/// local address it used to make the connection, which is the local machine's
/// main IPv4 address.
///
/// If the local machine does not have an IPv4 address, or if an error occurs
/// while creating the socket or making the connection, this function returns an
/// error.
///
/// This function does not support IPv6.
pub fn get_local_ip() -> Result<String> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("8.8.8.8:80")?;

    match socket.local_addr()? {
        SocketAddr::V4(addr) => Ok(format!("{}", *addr.ip())),
        SocketAddr::V6(_) => Err(anyhow!("IPv6 not supported")),
    }
}
