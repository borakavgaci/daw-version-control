use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::models::Project;

pub fn project_metadata_path(project_dir: &Path) -> PathBuf {
    project_dir.join("project.json")
}

pub fn read_project_metadata(metadata_path: &Path) -> io::Result<Project> {
    let file = fs::File::open(metadata_path)?;
    serde_json::from_reader(file).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

pub fn write_project_metadata(project_dir: &Path, project: &Project) -> io::Result<()> {
    fs::create_dir_all(project_dir)?;

    let metadata_path = project_metadata_path(project_dir);
    let file = fs::File::create(metadata_path)?;

    serde_json::to_writer_pretty(file, project)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

pub fn list_project_metadata(projects_root: &Path) -> io::Result<Vec<Project>> {
    if !projects_root.exists() {
        return Ok(Vec::new());
    }

    let mut projects = Vec::new();

    for entry_result in fs::read_dir(projects_root)? {
        let entry = entry_result?;
        let metadata_path = project_metadata_path(&entry.path());

        if metadata_path.is_file() {
            projects.push(read_project_metadata(&metadata_path)?);
        }
    }

    Ok(projects)
}
