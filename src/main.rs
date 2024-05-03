mod http_server;
mod load_conf;
mod cli;
mod gui;

use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--cli" {
        cli::run().await;
    } else {
        gui::run().await.unwrap();
    }
}
