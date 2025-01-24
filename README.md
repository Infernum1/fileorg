# File Organizer

File Organizer is a command-line utility written in Rust that organizes files in a directory based on their types (extensions) into respective subdirectories. It provides flexibility with options for copying or moving files, handling hidden files, and organizing files without extensions.

## Features

- Creates subdirectories based on file extensions and moves files into corresponding directories.
- Handles files without extensions by placing them in a custom directory.
- Option to include or exclude hidden files during organization.
- Supports both moving and copying files based on user preference.
- Logs all operations to a specified log file for reference.

## Installation

### From [crates.io](__https://crates.io__)

You can install the File Organizer directly from [crates.io](__https://crates.io/crates/fileorg__) using Cargo:

```sh
cargo install fileorg
```

### From source

Alternatively, you can build and install the application from the source code:

1. Clone the repository:

   ```sh
   git clone https://github.com/Infernum1/fileorg
   ```

2. Navigate to the project directory:

   ```sh
   cd fileorg
   ```

3. Build the project using Cargo:

   ```sh
   cargo build --release
   ```

## Usage

To organize files in a directory, run the following command:

### If installed from [crates.io](__https://crates.io__):

```sh
fileorg <directory_path> [OPTIONS]
```

### If built from source:

```sh
./target/release/fileorg <directory_path> [OPTIONS]
```

## Options
| Option | Description | Default |
|----------|----------|----------|
| --copy   | Copy files instead of moving them   | False   |
| --log_file   | Specify the log file path   | "file_organizer.log"   |
| --config    | Provide a path to a configuration file (`.toml` format)   | None   |
| --help, -h | Print help| - |
| --version, -V| Print version| - |

### NOTE
If you are using a config file (`--config` flag) in the CLI, all other flags will be ignored. If you would like to only use the `--copy` and `--log_file` flags individually, then there is no need to create a config file.
The config file only allows more control over handling hidden files and miscellaneous stuff.


## Configuration File

You can customize the behavior of the File Organizer by providing a configuration file (e.g., `config.toml`). The following fields are supported:

**`copy`**: Set to `true` to copy files instead of moving them. Defaults to false.

**`include_hidden`**: Set to `true` to include hidden files in the organization. Defaults to `false`.

**`others_directory`**: Specify the name of the directory for files without extensions. Defaults to `"Others"`.

**`log_file`**: Specify the path to the log file where operations will be recorded. Defaults to `"file_organizer.log"`.

### Example Configuration File

```toml
copy = true
include_hidden = false
others_directory = "Others"
log_file = "file_organizer.log"
```

### Using a Configuration File

- To use a configuration file, create a `config.toml` file with the format as suggested above, and feed it to the `--config` flag in the CLI.


All operations (e.g., file moves, copies, skips) are logged in the file specified in the configuration.

