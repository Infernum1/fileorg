use clap::{Arg, Command};
use fileorg::{organise_files, Config};
use std::fs;

fn main() {
    // Argument parsing
    let matches = Command::new("File Organizer")
        .version("0.1.1")
        .author("Infernum <infernum1212@gmail.com>")
        .about("Organizes files in a directory by their extensions")
        .arg(
            Arg::new("directory")
                .help("The directory to organize")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("copy")
                .long("copy")
                .help("Copy files instead of moving them (do not use this flag if you are providing a config file)")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("log_file")
                .long("log_file")
                .help("Path to the log file (do not use this flag if you are providing a config file)")
                .default_value("file_organizer.log"),
        )
        .arg(
            Arg::new("config")
                .long("config")
                .help("Path to a configuration file")
                .value_parser(clap::value_parser!(String)),
        )
        .override_usage(
            "Usage: fileorg <directory> [OPTIONS]\n\nExample:\n  fileorg ./test_dir --copy --config config.toml --log_file log.txt",
        )
        .get_matches_from({
            let args: Box<dyn Iterator<Item = String>> = if std::env::args().len() > 1 {
                Box::new(std::env::args())
            } else {
                let first_arg = std::env::args().next().unwrap_or_else(|| String::from("fileorg"));
                Box::new(vec![first_arg, "--help".to_string()].into_iter())
            };
            args
        });

    // Get the directory to organize
    let directory_to_organize = matches.get_one::<String>("directory").unwrap();

    // Get the log file path
    let log_file = matches.get_one::<String>("log_file").unwrap().to_string();

    // Load configuration file (if provided)
    let mut config = Config {
        copy: matches.get_flag("copy"),
        include_hidden: false,
        others_directory: "Others".to_string(),
        log_file,
    };

    if let Some(config_path) = matches.get_one::<String>("config") {
        match fs::read_to_string(config_path) {
            Ok(contents) => {
                config = toml::from_str(&contents).unwrap_or_else(|_| {
                    eprintln!("Invalid configuration file format.");
                    config
                });
            }
            Err(err) => {
                eprintln!("Failed to read configuration file: {}", err);
            }
        }
    }

    // Organize files
    if let Err(err) = organise_files(directory_to_organize, &config) {
        eprintln!("Error organizing files: {}", err);
    }
}
