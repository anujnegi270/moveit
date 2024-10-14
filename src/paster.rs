use std::env;
use std::fs;
use std::path::PathBuf;

fn run() {
    // Get the user's home directory
    let home_directory = env::var("HOME").or_else(|_| env::var("USERPROFILE")).unwrap();

    // Specify source files
    let source_files = vec![
        format!("{}/source/file1.txt", home_directory),
        format!("{}/source/file2.txt", home_directory),
    ];

    // Get target folder name from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <target_folder_name>", args[0]);
        std::process::exit(1);
    }
    let target_folder_name = &args[1];
    let destination_folder = format!("{}/{}", home_directory, target_folder_name);

    // Create the destination folder if it doesn't exist
    fs::create_dir_all(&destination_folder).expect("Failed to create destination folder");

    // Copy each file
    for file in source_files {
        match fs::copy(&file, &destination_folder) {
            Ok(_) => println!("Copied: {} to {}", file, destination_folder),
            Err(e) => eprintln!("Failed to copy {}: {}", file, e),
        }
    }

    println!("File copy operation completed!");
}
