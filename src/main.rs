use ports::Ports;
use ports::database::Protocol;

#[tokio::main]
async fn main() {
    let ports = Ports::default();
    let open_ports = ports.is_ports_open(0..=65535).await;

    for service in open_ports {
        let Some(service) = service else { continue };

        println!(
            "{} / {}: p:{:?} f:{}",
            &service.name, &service.port, &service.protocol, &service.frequency
        );
    }
}
