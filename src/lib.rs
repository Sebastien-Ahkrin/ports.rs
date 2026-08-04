use std::ops::RangeInclusive;
use tokio::{
    net::TcpStream,
    time::{Duration, timeout},
};

mod database;

struct Ports {
    duration: Duration,
}

impl Default for Ports {
    fn default() -> Self {
        Self {
            duration: Duration::from_millis(100),
        }
    }
}

impl Ports {
    fn new(duration: Duration) -> Self {
        Self { duration }
    }

    pub async fn is_port_open(&self, port: u16) -> bool {
        let connection = timeout(self.duration, TcpStream::connect(("127.0.0.1", port))).await;

        match connection {
            Ok(result) => result.is_ok(),
            Err(_) => false,
        }
    }

    pub async fn is_ports_open(&self, ports: RangeInclusive<u16>) -> Vec<u16> {
        let mut result = Vec::new();

        for port in ports {
            match self.is_port_open(port).await {
                true => result.push(port),
                false => continue,
            };
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;

    #[tokio::test]
    async fn mysql_exists() {
        assert_eq!(Ports::default().is_port_open(3306).await, true);
    }

    #[tokio::test]
    async fn multiple_port_open() {
        let first_listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let second_listener = TcpListener::bind("127.0.0.1:0").unwrap();

        let first_port = first_listener.local_addr().unwrap().port();
        let second_port = second_listener.local_addr().unwrap().port();

        assert_eq!(
            Ports::default()
                .is_ports_open(first_port..=second_port)
                .await,
            vec![first_port, second_port]
        );
    }
}
