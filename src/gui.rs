use std::io;

use gtk::prelude::*;
use gtk::{Button, Window, WindowType};

use crate::http_server;
use tokio;

pub async fn run() -> io::Result<()> {
    // Initialize GTK
    if gtk::init().is_err() {
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to initialize GTK"));
    }

    // Create main window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Diluvio HTTP Server");
    window.set_default_size(400, 300);

    // Create buttons
    let start_button = Button::with_label("Start Server");
    let stop_button = Button::with_label("Stop Server");
    let status_button = Button::with_label("Get Status");
    let logs_button = Button::with_label("Show Logs");
    let options_button = Button::with_label("Change Options");

    // Add buttons to window
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    vbox.add(&start_button);
    vbox.add(&stop_button);
    vbox.add(&status_button);
    vbox.add(&logs_button);
    vbox.add(&options_button);
    window.add(&vbox);

    // Connect signals
    start_button.connect_clicked(move |_| {
        println!("Starting Diluvio HTTP Server...");
        let start_arg = "127.0.0.1:3000".to_string();
        let fut = async move {
            if let Err(e) = http_server::start_http_server(&start_arg).await {
                eprintln!("Error starting server: {}", e);
            }
        };
        tokio::spawn(fut);
    });

    stop_button.connect_clicked(|_| {
        println!("Stopping Diluvio HTTP Server...");
        // Code to stop the server
    });

    status_button.connect_clicked(|_| {
        println!("Getting status of Diluvio HTTP Server...");
        // Code to get server status
    });

    logs_button.connect_clicked(|_| {
        println!("Showing logs of Diluvio HTTP Server...");
        // Code to show server logs
    });

    options_button.connect_clicked(|_| {
        println!("Changing options of Diluvio HTTP Server...");
        // Code to change server options
    });

    // Quit GTK when the window is closed
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Show all widgets
    window.show_all();

    // Run GTK main loop
    gtk::main();

    Ok(())
}
