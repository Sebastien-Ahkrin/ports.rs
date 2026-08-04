use ports::Ports;

#[tokio::main]
async fn main() {
    let open_ports = Ports::default().is_ports_open(0..=65532).await;

    for (name, port, open) in open_ports {
        println!("{} -> {}: {}", name, port, open);
    }
}
