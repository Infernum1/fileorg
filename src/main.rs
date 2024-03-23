use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a directory argument is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <directory>", args[0]);
        return;
    }

    // Organize files in the specified directory
    let directory_to_organize = &args[1];
    if let Err(err) = organize_files(directory_to_organize) {
        eprintln!("Error: {}", err);
    }
}

fn organize_files(directory: &str) -> std::io::Result<()> {
    // Get a list of entries in the directory
    let entries = fs::read_dir(directory)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recursively organize files in subdirectories
            organize_files(&path.display().to_string())?;
        } else {
            // Get the file extension
            if let Some(extension) = path.extension() {
                // Create the directory corresponding to the file extension if it doesn't exist
                let destination_directory = Path::new(directory).join(extension).with_extension(""); // Remove the extension for directory name

                if !destination_directory.exists() {
                    fs::create_dir(&destination_directory)?;
                }

                // Move the file to the destination directory
                let file_name = path.file_name().unwrap();
                let destination_file_path = destination_directory.join(file_name);
                fs::rename(&path, &destination_file_path)?;
                println!(
                    "Moved {} to {}",
                    file_name.to_string_lossy(),
                    destination_directory.display()
                );
            }
        }
    }
    Ok(())
}
