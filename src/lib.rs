use std::ops::RangeInclusive;
use tokio::{
    net::TcpStream,
    time::{Duration, timeout},
};
use crate::database::{Database, Protocol};

pub mod database;

pub struct Ports {
    database: Database,
    duration: Duration,
}

impl Default for Ports {
    fn default() -> Self {
        Self {
            database: Database::new(),
            duration: Duration::from_millis(100),
        }
    }
}

impl Ports {
    fn new(duration: Duration) -> Self {
        Self {
            database: Database::new(),
            duration,
        }
    }

    pub async fn is_port_open(&self, port: u16, protocol: Protocol) -> (String, bool) {
        let service = self.database.service_by_port(&(port, protocol));
        let connection = timeout(self.duration, TcpStream::connect(("127.0.0.1", port))).await;

        (
            service
                .map(|service| service.name.clone())
                .unwrap_or_else(|| "unknown".to_string()),
            match connection {
                Ok(result) => result.is_ok(),
                Err(_) => false,
            },
        )
    }

    pub async fn is_ports_open(&self, ports: RangeInclusive<u16>, protocol: Protocol) -> Vec<(String, u16, bool)> {
        let mut result = Vec::new();

        for port in ports {
            match self.is_port_open(port, protocol.clone()).await {
                (name, true) => result.push((name, port, true)),
                (_, false) => continue,
            };
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mysql_exists() {
        assert_eq!(
            Ports::default().is_port_open(3306, Protocol::Tcp).await,
            (String::from("mysql"), true)
        );
    }
}
