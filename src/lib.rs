//! The core library module for the File Organizer.
//! It provides the configuration structure and the file organization logic.

use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;
use std::fs::{self, OpenOptions};
use std::io::Write;
pub mod gui;

use std::path::Path;
use std::time::Instant;
use chrono::Local;

/// Configuration options for organizing files.
/// Users can specify whether to copy files instead of moving them,
/// whether to include hidden files, the name of the directory to store files
/// without extensions, and the path to the log file.
#[derive(Deserialize)]
pub struct Config {
    /// If true, files will be copied; otherwise, they will be moved.
    pub copy: bool,
    /// If true, hidden files will be included.
    pub include_hidden: bool,
    /// Directory name where files without extensions will be stored.
    pub others_directory: String,
    /// Path to the log file where operations are recorded.
    pub log_file: String,
}

/// Organizes files in the given directory according to their file extensions.
///
/// Files are either moved or copied based on the configuration provided.
/// Files without an extension are placed in a user-defined directory.
/// Progress and timing information is logged to the specified log file.
///
/// # Arguments
///
/// * `directory` - The directory containing files to organize.
/// * `config` - A reference to a `Config` structure with operation settings.
///
/// # Errors
///
/// Returns an `std::io::Error` if an error occurs during file system operations.
pub fn organise_files(directory: &str, config: &Config) -> std::io::Result<()> {
    // Start measuring total operation time.
    let start_time = Instant::now();

    // Open or create the log file.
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&config.log_file)?;

    // List items in the directory.
    let items: Vec<_> = fs::read_dir(directory)?.collect();
    let pb_style = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
        .unwrap()
        .progress_chars("█▉▍ ");

    let pb = ProgressBar::new(items.len() as u64);
    pb.set_style(pb_style);

    // Iterate over each file/directory in the given directory.
    for item in items {
        let item = item?;
        let path = item.path();

        if path.is_dir() {
            writeln!(log_file, "Skipping subdirectory: {}", path.display())?;
            pb.inc(1);
            continue;
        }

        // Skip hidden files if not included.
        if !config.include_hidden && path.file_name().unwrap().to_string_lossy().starts_with('.') {
            writeln!(log_file, "Skipping hidden file: {}", path.display())?;
            pb.inc(1);
            continue;
        }

        // Handle files with extensions.
        if let Some(extension) = path.extension() {
            let extension = extension.to_string_lossy().to_lowercase();
            let destination_directory = Path::new(directory).join(&extension).with_extension("");

            if !destination_directory.exists() {
                fs::create_dir(&destination_directory)?;
            }

            // Skip if the file is already in the correct directory.
            if path.parent() == Some(destination_directory.as_path()) {
                pb.inc(1);
                continue;
            }

            // Measure time for the copy/move operation.
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
            // Handle files without extensions.
            let others_directory = Path::new(directory).join(&config.others_directory);

            if !others_directory.exists() {
                fs::create_dir(&others_directory)?;
            }

            let file_name = path.file_name().unwrap();
            let destination_file_path = others_directory.join(file_name);

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