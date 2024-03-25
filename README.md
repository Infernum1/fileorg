# Rust File Organizer

Rust File Organizer is a command-line utility written in Rust that organizes files in a directory based on their types (extensions) into respective subdirectories.

## Feature

- Creates subdirectories based on file extensions and moves files into corresponding directories.

## Installation

### From crates.io

You can install the File Organizer directly from [crates.io](https://crates.io/crates/fileorg) using Cargo:

```sh
cargo install fileorg
```

### From source

Alternatively, you can build and install the application from the source code:

1. Clone the repository:

   ```sh
   git clone https://github.com/your-username/rust-file-organizer.git
   ```

2. Navigate to the project directory:

   ```sh
   cd rust-file-organizer
   ```

3. Build the project using Cargo:

   ```sh
   cargo build --release
   ```

## Usage

To organize files in a directory, run the following command:

### If installed from crates.io:

```sh
fileorg <directory_path>
```

### If built from source:

```sh
./target/release/fileorg <directory_path>
```
