use crate::error::{Errors, Result};
use rand::prelude::*;
use std::{collections::HashSet, net::IpAddr, ops::RangeInclusive};

pub mod error;
mod utils;

const MIN_PORT: u16 = 1024;
const MAX_PORT: u16 = 65535;

//
pub enum Protocol {
    All,
    Tcp,
    Udp,
}

/// PortPicker is a simple library to pick a free port in the local machine.
///
/// It can be used to find a free port to start a server or any other use case.
///
/// #Examples:
///
/// ```
/// use random_port::PortPicker;
/// let port = PortPicker::new().pick().unwrap();
/// println!("The free port is {}", port);
/// ```
pub struct PortPicker {
    range: RangeInclusive<u16>,
    exclude: HashSet<u16>,
    protocol: Protocol,
    host: Option<String>,
    random: bool,
}

impl PortPicker {
    pub fn new() -> Self {
        PortPicker {
            range: MIN_PORT..=MAX_PORT,
            exclude: HashSet::new(),
            protocol: Protocol::All,
            host: None,
            random: false,
        }
    }

    /// Specifies the range of ports to check. Must be in the range `1024..=65535`. E.g. `port_range(1024..=65535)`.
    pub fn port_range(mut self, range: RangeInclusive<u16>) -> Self {
        self.range = range;
        self
    }

    /// Specifies the ports to exclude.
    pub fn execlude(mut self, exclude: HashSet<u16>) -> Self {
        self.exclude = exclude;
        self
    }

    /// Specifies a port to exclude.
    pub fn execlude_add(mut self, port: u16) -> Self {
        self.exclude.insert(port);
        self
    }

    /// Specifies the protocol to check, Default is `Protocol::All`. Can be either `Protocol::Tcp`, `Protocol::Udp` or `Protocol::All`.
    pub fn protocol(mut self, protocol: Protocol) -> Self {
        self.protocol = protocol;
        self
    }

    /// Specifies the host to check. Can be either an Ipv4 or Ipv6 address.
    /// If not specified, will checks availability on all local addresses defined in the system.
    pub fn host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }

    /// Specifies whether to pick a random port from the range.
    /// If not specified, will pick the first available port from the range.
    pub fn random(mut self, random: bool) -> Self {
        self.random = random;
        self
    }

    fn random_port(&self, ip_addrs: HashSet<IpAddr>) -> Result<u16> {
        let mut rng = rand::thread_rng();
        let len = self.range.len();
        for _ in 0..len {
            let port = rng.gen_range(*self.range.start()..=*self.range.end());
            if self.exclude.contains(&port) {
                continue;
            }
            if utils::is_free_in_hosts(port, &ip_addrs, &self.protocol) {
                return Ok(port);
            }
        }
        Err(Errors::NoAvailablePort)
    }

    fn get_port(&self, ip_addrs: HashSet<IpAddr>) -> Result<u16> {
        for port in self.range.clone() {
            if self.exclude.contains(&port) {
                continue;
            }
            if utils::is_free_in_hosts(port, &ip_addrs, &self.protocol) {
                return Ok(port);
            }
        }
        Err(Errors::NoAvailablePort)
    }

    pub fn pick(&self) -> Result<u16> {
        // check params
        if self.range.is_empty() {
            return Err(Errors::InvalidOption(
                "The start port must be less than or equal to the end port".to_string(),
            ));
        }
        if *self.range.start() < MIN_PORT || *self.range.end() > MAX_PORT {
            return Err(Errors::InvalidOption(format!(
                "The port range must be between {} and {}",
                MIN_PORT, MAX_PORT
            )));
        }

        let mut ip_addrs: HashSet<IpAddr> = HashSet::new();
        if let Some(host) = &self.host {
            if let Ok(ip_addr) = host.parse::<IpAddr>() {
                ip_addrs.insert(ip_addr);
            } else {
                return Err(Errors::InvalidOption(format!(
                    "The host {} is not a valid IP address",
                    host
                )));
            }
        } else {
            ip_addrs = utils::get_local_hosts();
        }
        if self.random {
            self.random_port(ip_addrs)
        } else {
            self.get_port(ip_addrs)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_port_picker() {
        let port = PortPicker::new().pick().unwrap();
        assert!(port >= MIN_PORT && port <= MAX_PORT);

        let result = PortPicker::new().port_range(3000..=4000).pick();
        assert!(result.is_ok());
        let port = result.unwrap();
        assert!(port >= 3000 && port <= 4000);
    }
}
