use std::collections::HashMap;
use std::str::FromStr;

const NMAP_SERVICES: &str = include_str!("nmap-services.txt");

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Protocol {
    Tcp,
    Udp,
    Sctp,
}

#[derive(Debug)]
pub struct Service {
    pub name: String,
    pub port: u16,
    pub protocol: Protocol,
}

pub struct Database {
    /// (port, Tcp/Udp) -> Service
    services: HashMap<(u16, Protocol), Service>,
}

fn parse_port_protocol(data: &str) -> (u16, Protocol) {
    let mut split = data.split("/");

    let port = split.next().expect("cannot parse port");
    let protocol = split.next().expect("cannot parse protocol");

    (
        u16::from_str(port).expect("cannot get u16 from str"),
        match protocol {
            "tcp" => Protocol::Tcp,
            "udp" => Protocol::Udp,
            "sctp" => Protocol::Sctp,
            _ => panic!("unknown protocol: {:?} (data = {:?})", protocol, data),
        },
    )
}

impl Default for Database {
    fn default() -> Self {
        let mut services = HashMap::new();

        for line in NMAP_SERVICES.lines() {
            let line = line.trim();

            // remove header (commentary)
            if line.starts_with('#') {
                continue;
            }

            // cspmlockmgr	1272/tcp	0.000380 => [cspmlockmgr, 1272/tcp, 0.000380
            let mut parts = line.split_whitespace();

            let Some(name) = parts.next() else { continue };
            let Some(port_protocol) = parts.next() else {
                continue;
            };

            let (port, protocol) = parse_port_protocol(&port_protocol);

            services.insert(
                (port, protocol),
                Service {
                    name: name.to_string(),
                    port,
                    protocol,
                },
            );
        }

        Self { services }
    }
}

impl Database {
    pub fn lookup(&self, port: u16, protocol: Protocol) -> Option<&Service> {
        self.services.get(&(port, protocol))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // mysql	3306/tcp	0.045390
    #[test]
    fn get_mysql_from_database() {
        let database = Database::default();
        let service = database.lookup(3306, Protocol::Tcp);

        assert!(service.is_some());
        assert_eq!(service.unwrap().name, "mysql");
        assert_eq!(service.unwrap().port, 3306);
        assert_eq!(service.unwrap().protocol, Protocol::Tcp);
    }
}
