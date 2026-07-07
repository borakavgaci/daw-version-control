use std::io;
use std::path::{Path, PathBuf};

use crate::file_service;
use crate::models::Project;

pub fn next_version_number(project: &Project) -> u32 {
    project
        .versions
        .iter()
        .map(|version| version.number)
        .max()
        .unwrap_or(0)
        + 1
}

pub fn version_folder_name(version_number: u32) -> String {
    format!("v{version_number:03}")
}

pub fn version_relative_path(logic_project_name: &str, version_number: u32) -> PathBuf {
    PathBuf::from("versions")
        .join(version_folder_name(version_number))
        .join(logic_project_name)
}

pub fn copy_logic_project_version(source: &Path, destination: &Path) -> io::Result<()> {
    file_service::copy_dir_recursive(source, destination)
}
