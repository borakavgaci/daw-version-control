use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use tauri::{AppHandle, Manager};

pub fn is_logic_project_path(path: &Path) -> bool {
    let has_logic_extension = path
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("logicx"));

    path.is_dir() && has_logic_extension
}

pub fn ensure_directory(path: &Path) -> io::Result<()> {
    fs::create_dir_all(path)
}

pub fn projects_root(app_handle: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;
    let projects_dir = app_data_dir.join("projects");

    ensure_directory(&projects_dir).map_err(|error| error.to_string())?;

    Ok(projects_dir)
}

pub fn copy_dir_recursive(source: &Path, destination: &Path) -> io::Result<()> {
    if !source.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "source path must be a directory",
        ));
    }

    fs::create_dir_all(destination)?;

    for entry_result in fs::read_dir(source)? {
        let entry = entry_result?;
        let file_type = entry.file_type()?;
        let target_path = destination.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_recursive(&entry.path(), &target_path)?;
        } else if file_type.is_file() {
            fs::copy(entry.path(), target_path)?;
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "source directory contains an unsupported file type",
            ));
        }
    }

    Ok(())
}
