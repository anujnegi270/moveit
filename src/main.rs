use std::{env, path::PathBuf};

mod app;
mod files;

use crate::app::get_app;
use crate::files::{app_mkdir, copy_file, copy_folder, find_folder, get_available_drives};
use files::Files;

fn main() {
    let app = get_app();

    // Get the user's home directory
    let home_directory = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap();
    let downloads_directory = format!("{}/Downloads", home_directory);

    // Get the Azure Storage-xDpuWindows folder
    let azure_storage_directory = if let Some(az) = app.get_one::<String>("azure") {
        PathBuf::from(az)
    } else {
        // Get all available drives
        let drives = get_available_drives();
        let folder_name = "Storage-xDpuWindows";

        find_folder(&drives, folder_name)
            .unwrap_or_else(|| panic!("Storage-xDpuWindows folder not found"))
    };

    // Read source files from files.json
    // Read source files from embedded files.json
    let files_json = include_str!("config/files.json");
    let files: Files = serde_json::from_str(files_json).expect("Failed to parse files.json");

    // Get target folder name from command line argument
    let target_folder_name = if let Some(tar) = app.get_one::<String>("target") {
        tar.clone()
    } else {
        format!("{}", chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")).to_string()
    };
    let destination_folder = format!("{}\\{}", downloads_directory, target_folder_name);

    // Create the destination folder if it doesn't exist
    app_mkdir(PathBuf::from(&destination_folder));

    // Copy each file or folder
    for file in files.iter() {
        let file_name = file.name();
        let file_path = file.path();

        let source_path = azure_storage_directory.join(file_path);
        let destination_path = PathBuf::from(&destination_folder).join(file_name);

        if source_path.is_dir() {
            copy_folder(&source_path, &destination_path);
        } else {
            copy_file(&source_path, &destination_path);
        }
    }

    println!("File copy operation completed!");
}
