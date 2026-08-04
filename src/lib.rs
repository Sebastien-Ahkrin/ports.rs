use crate::database::{Database, Protocol, Service};
use std::ops::RangeInclusive;
use tokio::{
    net::TcpStream,
    time::{Duration, timeout},
};

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

    pub async fn is_port_open(&self, port: u16) -> (Option<&Service>, bool) {
        let service = self.database.service_by_port(&(port, Protocol::Tcp));
        let connection = timeout(self.duration, TcpStream::connect(("127.0.0.1", port))).await;

        (
            service,
            match connection {
                Ok(result) => result.is_ok(),
                Err(_) => false,
            },
        )
    }

    pub async fn is_ports_open(
        &self,
        ports: RangeInclusive<u16>,
    ) -> Vec<Option<&Service>> {
        let mut result = Vec::new();

        for port in ports {
            match self.is_port_open(port).await {
                (service, true) => result.push(service),
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
    async fn mysql_doesnt_exists() {
        let ports = Ports::default();
        let (service, open) = ports.is_port_open(3306).await;

        assert_eq!(
            (&service.unwrap().name, open),
            (&String::from("mysql"), false)
        );
    }
}
