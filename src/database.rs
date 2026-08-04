use std::collections::HashMap;
use std::str::SplitWhitespace;

const NMAP_SERVICES: &str = include_str!("../data/nmap-services.txt");

pub struct Database {
    services: HashMap<(u16, Protocol), Service>,
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum Protocol {
    Udp,
    Tcp,
    Sctp,
}

#[derive(Clone)]
pub struct Service {
    pub name: String,
    /// port are between [0..=65536]
    pub port: u16,
    /// Can be Udp, Tcp, Sctp
    pub protocol: Protocol,
    /// Open frequency
    pub frequency: f32,
}

impl Service {
    fn new(name: &str, port: u16, protocol: Protocol, frequency: f32) -> Self {
        Self {
            name: name.to_string(),
            port,
            protocol,
            frequency,
        }
    }
}

impl Database {
    pub fn new() -> Self {
        let mut services = HashMap::new();

        for line in NMAP_SERVICES.lines() {
            if line.starts_with("#") {
                continue;
            }

            let parts = line.split_whitespace();
            let service = parse_line_from_file(parts);

            let _ = &services.insert((service.clone().port, service.clone().protocol), service);
        }

        Self { services }
    }

    pub fn service_by_port(&self, port_protocol: &(u16, Protocol)) -> Option<&Service> {
        self.services.get(port_protocol)
    }
}

fn parse_line_from_file(mut line: SplitWhitespace<'_>) -> Service {
    let name = line.next().expect("Cannot parse line name");
    let port_protocol = line.next().expect("Cannot parse protocol");
    let frequency = line.next().expect("Cannot parse frequency");

    let (port, protocol) = parse_port_protocol(port_protocol);

    Service::new(
        name,
        port,
        protocol,
        frequency
            .parse::<f32>()
            .expect("Cannot parse frequency from &str"),
    )
}

fn parse_port_protocol(data: &str) -> (u16, Protocol) {
    let mut split = data.split("/");

    let port = split.next().expect("cannot parse port");
    let protocol = split.next().expect("cannot parse protocol");

    (
        port.parse::<u16>().expect("cannot parse port from &str"),
        match protocol {
            "udp" => Protocol::Udp,
            "tcp" => Protocol::Tcp,
            "sctp" => Protocol::Sctp,
            _ => panic!("Unknown protocol: {}", protocol),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mysql_exists() {
        let database = Database::new();
        let service = database.service_by_port(&(3306, Protocol::Tcp));
        assert!(service.is_some());
    }
}
