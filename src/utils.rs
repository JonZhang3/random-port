use std::{
    collections::HashSet,
    io::ErrorKind,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpListener, UdpSocket},
};

use network_interface::{NetworkInterface, NetworkInterfaceConfig};

use crate::Protocol;

/// Get all available network interfaces
pub(crate) fn get_local_hosts() -> HashSet<IpAddr> {
    let mut result = HashSet::<IpAddr>::new();
    result.insert(Ipv4Addr::UNSPECIFIED.into());
    result.insert(Ipv6Addr::UNSPECIFIED.into());
    let interfaces = NetworkInterface::show().unwrap();
    for interface in interfaces {
        for addr in interface.addr {
            result.insert(addr.ip());
        }
    }
    result
}

/// Check if a port is free in all hosts
pub(crate) fn is_free_in_hosts(port: u16, hosts: &HashSet<IpAddr>, protocol: &Protocol) -> bool {
    for host in hosts {
        if !is_free(port, host, protocol) {
            println!("Port {} is not free in {}", port, host);
            return false;
        }
    }
    true
}

/// Check if a port is free
pub(crate) fn is_free(port: u16, host: &IpAddr, protocol: &Protocol) -> bool {
    match protocol {
        Protocol::Tcp => is_free_tcp(port, host),
        Protocol::Udp => is_free_udp(port, host),
        Protocol::All => is_free_tcp(port, host) && is_free_udp(port, host),
    }
}

/// Check if a TCP port is free
pub(crate) fn is_free_tcp(port: u16, host: &IpAddr) -> bool {
    let socket_addr = SocketAddr::new(*host, port);
    let result = TcpListener::bind(socket_addr);
    if let Ok(_) = result {
        return true;
    }
    let err = result.unwrap_err();
    if err.kind() == ErrorKind::AddrNotAvailable || err.kind() == ErrorKind::InvalidInput {
            return true;
        }
    false
}

/// Check if a UDP port is free
pub(crate) fn is_free_udp(port: u16, host: &IpAddr) -> bool {
    let socket_addr = SocketAddr::new(*host, port);
    let result = UdpSocket::bind(socket_addr);
    if let Ok(_) = result {
        return true;
    }
    let err = result.unwrap_err();
    if err.kind() == ErrorKind::AddrNotAvailable || err.kind() == ErrorKind::InvalidInput {
        return true;
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_local_hosts() {
        let result = get_local_hosts();
        assert!(result.len() > 0);
    }
}
