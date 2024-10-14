# MoveIt

MoveIt is a Rust-based file management tool that copies files and folders from a specified source directory to a target directory. The source directory is typically an Azure Storage folder, and the target directory is specified via command line arguments.


## Getting Started

### Prerequisites

- Rust and Cargo installed. You can download them from [rust-lang.org](https://www.rust-lang.org/).

### Building the Project

To build the project, run the following command in the root directory:

```sh
cargo build
```
OR
```sh
cargo build --release
```

### Running the Project

To run the project, use the following command:

```sh
cargo run -- -a <path_to_azure_storage> -t <target_folder_name>
```
OR
```sh
cargo run -- --azure <path_to_azure_storage> --target <target_folder_name>
```
where, 
 - `--azure` or `-a`: Path to the Azure Storage folder. If not provided, the program will search for the Storage-xDpuWindows folder in available drives.
 - `--target` or `-t`: Name of the target folder. If not provided, the current timestamp will be used as the folder name.

## Contributing

Please create a PR if you want to commit your changes, regardless of any feature, bugs or issues. 
