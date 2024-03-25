use std::env;

use fileorg::organise_files;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if directory argument provided
    if args.len() < 2 {
        eprintln!("Usage: fileorg <directory>");
        return;
    }

    // Organize the files
    let directory_to_organize = &args[1];
    if let Err(err) = organise_files(directory_to_organize) {
        eprintln!("Error: {}", err);
    }
}
