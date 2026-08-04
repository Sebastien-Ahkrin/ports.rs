use ports::is_port_open;

fn main() {
    let is_open = is_port_open(8081);
    println!("{is_open}")
}
