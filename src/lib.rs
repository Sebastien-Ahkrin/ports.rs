use portlex::{Glossary, Line, Protocol};
use std::sync::LazyLock;
use tokio::net::{TcpStream, UdpSocket};
use tokio::time::{Duration, timeout};

static GLOSSARY: LazyLock<Glossary> = LazyLock::new(|| Glossary::default());

pub enum SupportedProtocol {
    Tcp,
    Udp,
}

impl From<SupportedProtocol> for Protocol {
    fn from(protocol: SupportedProtocol) -> Self {
        match protocol {
            SupportedProtocol::Tcp => Protocol::Tcp,
            SupportedProtocol::Udp => Protocol::Udp,
        }
    }
}

#[derive(Clone)]
pub struct Port {
    duration: Duration,
    url: String,
}

impl Default for Port {
    fn default() -> Self {
        Self {
            duration: Duration::from_millis(100),
            url: "127.0.0.1".to_string(),
        }
    }
}

impl Port {
    pub fn new(duration: Duration, url: &str) -> Self {
        Port {
            duration,
            url: url.to_string(),
        }
    }

    async fn tcp_connect(&self, port: u16) -> (Option<&Line>, bool) {
        let service = GLOSSARY.get_line(port, Protocol::Tcp);
        let connection = timeout(self.duration, TcpStream::connect((self.url.clone(), port))).await;

        (
            service,
            connection.map(|result| result.is_ok()).unwrap_or(false),
        )
    }

    pub async fn is_port_open(
        &self,
        port: u16,
        protocol: SupportedProtocol,
    ) -> (Option<&Line>, bool) {
        match protocol {
            SupportedProtocol::Tcp => self.tcp_connect(port).await,
            SupportedProtocol::Udp => panic!("Not implemented yet"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mysql_doesnt_exists() {
        let ports = Port::default();
        let (service, open) = ports.is_port_open(3306, SupportedProtocol::Tcp).await;

        assert_eq!(
            (&service.unwrap().name, open),
            (&String::from("mysql"), false)
        );
    }
}
