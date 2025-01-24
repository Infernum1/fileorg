use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::Instant;
use chrono::Local;

#[derive(Deserialize)]
pub struct Config {
    pub copy: bool,
    pub include_hidden: bool,
    pub others_directory: String,
    pub log_file: String,
}

pub fn organise_files(directory: &str, config: &Config) -> std::io::Result<()> {
    // Start measuring total operation time
    let start_time = Instant::now();

    // Open or create the log file
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&config.log_file)?;

    // List items in the directory
    let items: Vec<_> = fs::read_dir(directory)?.collect();
    let pb_style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
        .unwrap()
        .progress_chars("█▉▍ ");

    let pb = ProgressBar::new(items.len() as u64);
    pb.set_style(pb_style);

    for item in items {
        let item = item?;
        let path = item.path();

        if path.is_dir() {
            writeln!(log_file, "Skipping subdirectory: {}", path.display())?;
            pb.inc(1);
            continue;
        }

        // Handle hidden files based on config
        if !config.include_hidden && path.file_name().unwrap().to_string_lossy().starts_with('.') {
            writeln!(log_file, "Skipping hidden file: {}", path.display())?;
            pb.inc(1);
            continue;
        }

        // Get file extension
        if let Some(extension) = path.extension() {
            let extension = extension.to_string_lossy().to_lowercase();
            let destination_directory = Path::new(directory).join(&extension).with_extension("");

            if !destination_directory.exists() {
                fs::create_dir(&destination_directory)?;
            }

            // Skip if file is already in the correct directory
            if path.parent() == Some(destination_directory.as_path()) {
                pb.inc(1);
                continue;
            }

            // Start measuring time for copy/move operation
            let start_file_time = Instant::now();

            let file_name = path.file_name().unwrap();
            let destination_file_path = destination_directory.join(file_name);

            if config.copy {
                fs::copy(&path, &destination_file_path)?;
                let elapsed_time = start_file_time.elapsed();
                writeln!(
                    log_file,
                    "Copied {} to {} in {:.2?}",
                    file_name.to_string_lossy(),
                    destination_directory.display(),
                    elapsed_time
                )?;
            } else {
                fs::rename(&path, &destination_file_path)?;
                let elapsed_time = start_file_time.elapsed();
                writeln!(
                    log_file,
                    "Moved {} to {} in {:.2?}",
                    file_name.to_string_lossy(),
                    destination_directory.display(),
                    elapsed_time
                )?;
            }
        } else {
            // Handle files without extensions
            let others_directory = Path::new(directory).join(&config.others_directory);

            if !others_directory.exists() {
                fs::create_dir(&others_directory)?;
            }

            let file_name = path.file_name().unwrap();
            let destination_file_path = others_directory.join(file_name);

            // Start measuring time for copy or move operation
            let start_file_time = Instant::now();

            if config.copy {
                fs::copy(&path, &destination_file_path)?;
                let elapsed_time = start_file_time.elapsed();
                writeln!(
                    log_file,
                    "Copied {} to {} in {:.2?}",
                    file_name.to_string_lossy(),
                    others_directory.display(),
                    elapsed_time
                )?;
            } else {
                fs::rename(&path, &destination_file_path)?;
                let elapsed_time = start_file_time.elapsed();
                writeln!(
                    log_file,
                    "Moved {} to {} in {:.2?}",
                    file_name.to_string_lossy(),
                    others_directory.display(),
                    elapsed_time
                )?;
            }
        }

        pb.inc(1);
    }

    let total_elapsed_time = start_time.elapsed();

    let current_time = Local::now();
    writeln!(
        log_file,
        "Operation completed successfully at {} on {}. Total time taken: {:.2?}, {:?} files processed.\n",
        current_time.format("%H:%M:%S").to_string(),
        current_time.format("%A, %B %d, %Y").to_string(),
        total_elapsed_time,
        pb.length().unwrap_or(0),
    )?;
    
    pb.finish_with_message("Done!");
    Ok(())
}
