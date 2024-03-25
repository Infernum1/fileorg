use std::fs;
use std::path::Path;

pub fn organise_files(directory: &str) -> std::io::Result<()> {
    // List of items in the directories
    let items = fs::read_dir(directory)?;

    for item in items {
        let item = item?;
        let path = item.path();

        if path.is_dir() {
            // Path is a directory, so we recursively organise the files inside
            organise_files(&path.display().to_string())?;
        } else {
            // Get file extension
            if let Some(extension) = path.extension() {
                // Create directory corresponding to the file extension (if doesnt exist)
                let destination_directory = Path::new(directory).join(extension).with_extension("");

                if !destination_directory.exists() {
                    fs::create_dir(&destination_directory)?;
                }

                // Move file to destination directory
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
