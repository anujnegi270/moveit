use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Deserialize)]
pub(crate) struct File {
    name: String,
    path: String,
}

#[derive(Deserialize)]
pub(crate) struct Files {
    files: Vec<File>,
}

impl File {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn path(&self) -> &str {
        &self.path
    }
}

impl Files {
    pub(crate) fn iter(&self) -> impl Iterator<Item = &File> {
        self.files.iter()
    }
}

pub(crate) fn app_mkdir(path: PathBuf) {
    match fs::create_dir_all(&path) {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to create directory {}: {}", path.display(), e),
    }
}

pub(crate) fn get_available_drives() -> Vec<String> {
    let mut drives = Vec::new();
    for drive in 'A'..='Z' {
        let drive_path = format!("{}:\\", drive);
        if PathBuf::from(&drive_path).exists() {
            drives.push(drive_path);
        }
    }
    drives
}

pub(crate) fn find_folder(drives: &[String], folder_name: &str) -> Option<PathBuf> {
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

pub(crate) fn copy_file(source_path: &Path, destination_path: &Path) {
    match fs::copy(source_path, destination_path) {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to copy file {}: {}", source_path.display(), e),
    }
}

pub(crate) fn copy_folder(source_path: &Path, destination_path: &Path) {
    for entry in WalkDir::new(source_path).min_depth(1) {
        let entry = entry.expect("Failed to read directory entry");
        let relative_path = entry
            .path()
            .strip_prefix(source_path)
            .expect("Failed to get relative path");
        let dest_path = destination_path.join(relative_path);

        if entry.file_type().is_file() {
            if let Some(parent) = dest_path.parent() {
                app_mkdir(parent.to_path_buf());
            }
            copy_file(entry.path(), &dest_path);
        } else if entry.file_type().is_dir() {
            app_mkdir(dest_path);
        }
    }
}
