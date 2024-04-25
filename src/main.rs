mod http_server;
mod dns_server;
mod dhcp_server;
mod load_conf;

use std::io;

fn main() -> io::Result<()> {
    // Start HTTP server on localhost:3000
    // Start DNS server on localhost:53
    // TODO: Load configuration from file
    // TODO: Start DNS server
    // TODO: Start DHCP server


    http_server::start_http_server("127.0.0.1:3000")
    // dns_server::start_dns_server("127.0.0.1:53")
    // dhcp_server::start_dhcp_server("127.0.0.1:67")

}
