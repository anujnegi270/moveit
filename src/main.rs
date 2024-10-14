use std::env;
use std::fs;
use std::path::PathBuf;

mod files;
use clap::{Arg, Command};
use files::Files;
use walkdir::WalkDir;

fn get_available_drives() -> Vec<String> {
    let mut drives = Vec::new();
    for drive in 'A'..='Z' {
        let drive_path = format!("{}:\\", drive);
        if PathBuf::from(&drive_path).exists() {
            drives.push(drive_path);
        }
    }
    drives
}

fn find_folder(drives: &[String], folder_name: &str) -> Option<PathBuf> {
    for drive in drives {
        for entry in WalkDir::new(drive).min_depth(1).max_depth(3) {
            let entry = if let Ok(entry) = entry {
                entry
            } else {
                continue;
            };
            if entry.file_type().is_dir() && entry.file_name() == folder_name {
                return Some(entry.path().to_path_buf());
            }
        }
    }
    None
}

fn main() {
    let matches = Command::new("MoveIt")
        .version("1.0")
        .author("Anuj Negi <anujnegi@microsoft.com>")
        .about("Copies files to a specified folder")
        .arg(
            Arg::new("azure")
                .short('a')
                .long("azure")
                .help("To copy from your Azure Storage-xDpuWindows folder"),
        )
        .arg(
            Arg::new("target")
                .help("Sets the target folder name")
                .long("target")
                .short('t'),
        )
        .get_matches();

    // Get the user's home directory
    let home_directory = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap();
    let downloads_directory = format!("{}/Downloads", home_directory);

    // Get the Azure Storage-xDpuWindows folder
    let azure_storage_directory = if let Some(az) = matches.get_one::<String>("azure") {
        PathBuf::from(az)
    } else {
        // Get all available drives
        let drives = get_available_drives();
        let folder_name = "Storage-xDpuWindows";

        find_folder(&drives, folder_name)
            .unwrap_or_else(|| panic!("Storage-xDpuWindows folder not found"))
    };

    // Read source files from files.json
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let files_json_path = current_dir.join("src/files.json");
    println!("Reading files from: {}", files_json_path.display());

    let files_json = fs::read_to_string(files_json_path).expect("Failed to read files.json");
    let files: Files = serde_json::from_str(&files_json).expect("Failed to parse files.json");

    // Get target folder name from command line argument
    let target_folder_name = if let Some(tar) = matches.get_one::<String>("target") {
        tar.clone()
    } else {
        format!("{}", chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")).to_string()
    };
    let destination_folder = format!("{}\\{}", downloads_directory, target_folder_name);

    // Create the destination folder if it doesn't exist
    fs::create_dir_all(&destination_folder).expect("Failed to create destination folder");

    // Copy each file or folder
    for file in files.iter() {

        let file_name = file.name();
        let file_path = file.path();

        let source_path = azure_storage_directory.join(file_path);
        let destination_path = PathBuf::from(&destination_folder).join(file_name);
        
        if source_path.is_dir() {

            // Copy directory recursively
            for entry in WalkDir::new(&source_path).min_depth(1) {
                let entry = entry.expect("Failed to read directory entry");
                    let relative_path = entry
                    .path()
                    .strip_prefix(&source_path)
                    .expect("Failed to get relative path");
                let dest_path = destination_path.join(relative_path);

                if entry.file_type().is_file() {
                    if let Some(parent) = dest_path.parent() {
                        fs::create_dir_all(parent).expect("Failed to create parent directory");
                    }
                    match fs::copy(entry.path(), &dest_path) {
                        Ok(_) => {},
                        Err(e) => eprintln!("Failed to copy file {}: {}", entry.path().display(), e),
                    }
                } else if entry.file_type().is_dir() {
                    match fs::create_dir_all(&dest_path) {
                        Ok(_) => {},
                        Err(e) => {
                            eprintln!("Failed to create directory {}: {}", dest_path.display(), e)
                        }
                    }
                }
            }
        }
        else {
            match fs::copy(&source_path, &destination_path) {
                Ok(_) => {},
                Err(e) => eprintln!("Failed to copy file {}: {}", source_path.display(), e),
            }
        }
    }

    println!("File copy operation completed!");
}
