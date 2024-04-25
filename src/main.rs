mod http_server;
mod dns_server;
mod dhcp_server;
mod load_conf;
use std::io;

fn main() -> io::Result<()> {
    // Start HTTP server on localhost:3000
    http_server::start_http_server("127.0.0.1:3000")
}
