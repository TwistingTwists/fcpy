# Rust File Copier

Rust File Copier is a command-line tool that searches for files containing a specific string in their names within a given directory (and its subdirectories) and copies the matching files to a destination folder. It provides options for case-sensitive or case-insensitive search.

## Features

- Recursive file search in the input directory and all subdirectories
- Case-sensitive or case-insensitive file name matching (case-insensitive by default)
- Copies matching files to a specified destination folder
- Preserves full path information in the copied file names

## Installation

1. Ensure you have Rust and Cargo installed on your system. If not, you can install them from [https://www.rust-lang.org/](https://www.rust-lang.org/).

2. Clone this repository:
   ```
   git clone https://github.com/yourusername/rust-file-copier.git
   cd rust-file-copier
   ```

3. Build the project:
   ```
   cargo build --release
   ```

The compiled binary will be available in the `target/release` directory.

## Usage

Run the program with the following command:

```
cargo run -- --input-folder <INPUT_FOLDER> --search-string <SEARCH_STRING> --dest-folder <DEST_FOLDER> [--case-sensitive]
```

Or, if you've built the release version:

```
./target/release/rust-file-copier --input-folder <INPUT_FOLDER> --search-string <SEARCH_STRING> --dest-folder <DEST_FOLDER> [--case-sensitive]
```

### Arguments:

- `--input-folder, -i`: The directory to search for files (required)
- `--search-string, -s`: The string to search for in file names (required)
- `--dest-folder, -d`: The directory where matching files will be copied (required)
- `--case-sensitive, -c`: Enable case-sensitive search (optional, default is case-insensitive)

### Examples:

1. Search for files containing "example" (case-insensitive) in the "documents" folder and copy them to "output":
   ```
   cargo run -- --input-folder documents --search-string example --dest-folder output
   ```

2. Perform a case-sensitive search for "Example" in the "projects" folder and copy matches to "found_files":
   ```
   cargo run -- --input-folder projects --search-string Example --dest-folder found_files --case-sensitive
   ```

## How it Works

1. The program walks through all files in the input directory and its subdirectories.
2. For each file, it checks if the file name contains the specified search string.
3. If a match is found, the file is copied to the destination folder.
4. The copied file's name includes its full original path (with "/" replaced by "_") to preserve uniqueness and path information.

## Dependencies

This project uses the following Rust crates:

- `clap`: For parsing command-line arguments
- `walkdir`: For recursively walking through directories