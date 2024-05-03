use crate::http_server;
use clap::{App, Arg};
use std::process;

pub async fn run() {
    let app = App::new("http-server")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Aguacero")
        .about("Diluvio HTTP Server")
        .arg(Arg::new("start")
            .long("start")
            .help("Start the HTTP server"))
        .arg(Arg::new("status")
            .long("status")
            .help("Get the status of the HTTP server"))
        .arg(Arg::new("stop")
            .long("stop")
            .help("Stop the HTTP server"))
        .arg(Arg::new("logs")
            .long("logs")
            .help("Show the logs of the HTTP server"))
        .arg(Arg::new("version")
            .long("version")
            .help("Display the version of the HTTP server"))
        .arg(Arg::new("config")
            .long("config")
            .help("Load configuration file"))
        .arg(Arg::new("edit")
            .long("edit")
            .help("Edit settings"));

    let matches = app.get_matches();

    if matches.is_present("start") {
        let start_arg = matches.value_of("start").unwrap();
        if let Err(e) = http_server::start_http_server(start_arg).await {
            eprintln!("Error starting server: {}", e);
            process::exit(1);
        }
    } else if matches.is_present("status") {
        //println!("{}", get_status());
    } else if matches.is_present("stop") {
        //stop_http_server();
    } else if matches.is_present("logs") {
        //println!("{}", get_logs());
    } else if matches.is_present("version") {
        //display_version();
    } else if matches.is_present("config") {
       // load_config();
    } else if matches.is_present("edit") {
        //edit_settings();
    } else {
        eprintln!("No command specified.");
        process::exit(1);
    }
}
