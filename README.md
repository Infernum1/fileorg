# File Organizer

![File Organizer Logo](assets/images/logo.png)

**File Organizer** is a cross-platform GUI utility written in Rust for automatically organizing files in a directory based on their file types. With a modern, user-friendly interface powered by [eframe](https://github.com/emilk/egui) and [egui](https://github.com/emilk/egui), this tool categorizes files by extension into dedicated subdirectories while providing flexible options for copying or moving files, including hidden files, and handling files without extensions.

---

## Table of Contents

- [Features](#features)
- [Installation](#installation)
  - [MSI Installer (Windows)](#msi-installer-windows)
  - [Cargo (crates.io)](#cargo-cratesio)
  - [Building from Source](#building-from-source)
- [Usage](#usage)

---

## Features

- **Automatic Organization:** Creates subdirectories based on file extensions and moves or copies files accordingly.
- **Custom Directory for Unrecognized Files:** Specify a custom folder name for files without extensions.
- **Hidden Files Handling:** Option to include or exclude hidden files during the organization process.
- **Copy or Move Files:** Choose whether to copy files or move them to the new organized structure.
- **Detailed Logging:** Logs all operations to a specified log file for auditing and troubleshooting.

---

## Installation

### MSI Installer (Windows)

Download the latest MSI installer from the [Releases](https://github.com/Infernum1/fileorg/releases) page and follow the installation wizard to install File Organizer on your Windows system.

### Cargo (crates.io)

Install File Organizer directly using Cargo:

```sh
cargo install fileorg
```

### Building From source

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

Just run the application without any arguments:

```sh
fileorg
```

This will launch the graphical user interface, allowing you to select the directory to organize, choose operation modes (copy or move), toggle hidden file inclusion, and specify log file and custom directory options.

