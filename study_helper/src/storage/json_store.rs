// src/storage/json_store.rs

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use crate::models::StudySet;

/// Save a single study set to the given file path (overwrites).
pub fn save_study_set_to_file(study_set: &StudySet, file_path: &Path) -> io::Result<()> {
    let json = serde_json::to_string_pretty(study_set).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut f = fs::File::create(file_path)?;
    f.write_all(json.as_bytes())?;
    // Log the save operation for debugging; the caller controls log level.
    log::debug!("Saved study set '{}' to {}", study_set.name(), file_path.display());
    Ok(())
}

/// Load a single study set from the given file path.
pub fn load_study_set_from_file(file_path: &Path) -> io::Result<StudySet> {
    let data = fs::read_to_string(file_path)?;
    let set: StudySet = serde_json::from_str(&data).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    log::debug!("Loaded study set '{}' from {}", set.name(), file_path.display());
    Ok(set)
}

/// Save a study set into a class folder under base_dir: base_dir / class_name / set_name.json
pub fn save_set_into_class_folder(base_dir: &Path, class_name: &str, set_name: &str, study_set: &StudySet) -> io::Result<PathBuf> {
    let mut path = base_dir.join(class_name);
    fs::create_dir_all(&path)?;
    path.push(format!("{}.json", sanitize_filename::sanitize(set_name)));
    save_study_set_to_file(study_set, &path)?;
    log::info!("Persisted set '{}' into class '{}' at {}", set_name, class_name, path.display());
    Ok(path)
}

/// Load all study sets from a class folder (all .json files)
pub fn load_sets_from_class_folder(base_dir: &Path, class_name: &str) -> io::Result<Vec<StudySet>> {
    let mut sets = Vec::new();
    let dir = base_dir.join(class_name);
    if !dir.exists() { return Ok(sets); }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let p = entry.path();
        if p.extension().and_then(|s| s.to_str()).map(|s| s.eq_ignore_ascii_case("json")).unwrap_or(false) {
            if let Ok(set) = load_study_set_from_file(&p) {
                sets.push(set);
            }
        }
    }
    log::info!("Loaded {} sets from class folder '{}'", sets.len(), class_name);
    Ok(sets)
}

/// Import a study set JSON file into a class folder (copy file)
pub fn import_set_file_to_class(base_dir: &Path, class_name: &str, src_file: &Path) -> io::Result<PathBuf> {
    let class_dir = base_dir.join(class_name);
    fs::create_dir_all(&class_dir)?;
    let file_name = src_file.file_name().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "source has no filename"))?;
    let dst = class_dir.join(file_name);
    fs::copy(src_file, &dst)?;
    Ok(dst)
}

/// Export a study set file out to a destination path (useful for sharing)
pub fn export_set_file(src_file: &Path, dst_file: &Path) -> io::Result<()> {
    if let Some(parent) = dst_file.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(src_file, dst_file)?;
    Ok(())
}

/// List class folders (subdirectories) under the given base directory.
pub fn list_class_folders(base_dir: &Path) -> io::Result<Vec<String>> {
    let mut classes = Vec::new();
    if !base_dir.exists() {
        return Ok(classes);
    }
    for entry in fs::read_dir(base_dir)? {
        let entry = entry?;
        let p = entry.path();
        if p.is_dir() {
            if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                classes.push(name.to_string());
            }
        }
    }
    Ok(classes)
}