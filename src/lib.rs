mod database;

use std::net::TcpStream;
use crate::database::{Database, Protocol};

/// Check if a specific port is already open
pub fn is_port_open(port: u16) -> bool {
    let connection = TcpStream::connect(("127.0.0.1", port));
    connection.is_ok()
}

/// Check if a specific port is already open and give the name too
pub fn is_port_open_with_name(port: u16) -> (bool, String) {
    let database = Database::default();
    let result = is_port_open(port);
    
    let service = database.lookup(port, Protocol::Tcp);
    assert!(service.is_some());
    
    let name = &service.unwrap().name;
    (result, name.clone())
}

/// Check if range of ports is allocated. Return a Vec<u16> with ports that is allocated
pub fn ports_open_in_range(ports: Vec<u16>) -> Vec<u16> {
    let mut allocated_ports = Vec::new();

    for port in ports {
        match is_port_open(port) {
            true => allocated_ports.push(port),
            false => continue,
        }
    }

    allocated_ports
}

/// Check if range of ports is allocated. Return a Vec<(bool, u16, String)> with (port, name) that is allocated
pub fn ports_open_in_range_with_name(ports: Vec<u16>) -> Vec<(u16, String)> {
    let mut allocated_ports = Vec::new();
    
    for port in ports {
        let result = is_port_open_with_name(port);
        match result {
            (true, name) => allocated_ports.push((port, name)),
            (false, name) => continue,
        }
    }
    
    allocated_ports
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;

    #[test]
    fn port_is_allocated() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();

        assert!(is_port_open(port));
    }

    #[test]
    fn port_is_not_allocated() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        drop(listener);

        assert!(!is_port_open(port));
    }

    #[test]
    fn port_open_in_range_is_allocated() {
        let first_listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let second_listener = TcpListener::bind("127.0.0.1:0").unwrap();

        let first_port = first_listener.local_addr().unwrap().port();
        let second_port = second_listener.local_addr().unwrap().port();

        assert_eq!(
            ports_open_in_range(vec![first_port, second_port]).len(),
            vec![first_port, second_port].len()
        );
    }

    #[test]
    fn only_one_port_is_allocated() {
        let first_listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let second_listener = TcpListener::bind("127.0.0.1:0").unwrap();

        let first_port = first_listener.local_addr().unwrap().port();
        let second_port = second_listener.local_addr().unwrap().port();

        drop(first_listener);

        assert_eq!(
            ports_open_in_range(vec![first_port, second_port]).len(),
            vec![second_port].len()
        );
    }

    #[test]
    fn zero_port_is_allocated() {
        let first_listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let second_listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let first_port = first_listener.local_addr().unwrap().port();
        let second_port = second_listener.local_addr().unwrap().port();

        drop(first_listener);
        drop(second_listener);

        assert_eq!(ports_open_in_range(vec![first_port, second_port]).len(), 0);
    }
}
