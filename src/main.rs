//! Entry point for the File Organizer application.

mod gui;
use fileorg::{organise_files, Config};

fn main() {
    if let Err(e) = gui::run_gui() {
        eprintln!("Error running GUI: {}", e);
    }
}