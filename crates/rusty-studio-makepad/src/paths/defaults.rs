use super::normalize::normalize_verbatim_path;
use std::path::{Path, PathBuf};

pub(crate) fn find_default_project_path() -> Option<PathBuf> {
    let current_dir = std::env::current_dir().ok()?;
    let candidates = [
        current_dir.join("examples/synthetic-studio-project.json"),
        current_dir.join("../../examples/synthetic-studio-project.json"),
        current_dir.join("../../../examples/synthetic-studio-project.json"),
    ];
    candidates.into_iter().find(|path| path.is_file())
}

pub(crate) fn default_project_working_copy_path(default_path: &Path) -> Result<PathBuf, String> {
    let default_path = normalize_verbatim_path(
        std::fs::canonicalize(default_path)
            .map_err(|error| format!("Default example resolve failed: {error}"))?,
    );
    let examples_dir = default_path
        .parent()
        .ok_or_else(|| "default example has no parent directory".to_string())?;
    let repo_root = examples_dir
        .parent()
        .ok_or_else(|| "default example is not inside the repo examples directory".to_string())?;
    let file_name = default_path
        .file_name()
        .ok_or_else(|| "default example has no file name".to_string())?;
    let working_dir = repo_root.join("examples-working");
    let working_path = working_dir.join(file_name);
    std::fs::create_dir_all(&working_dir)
        .map_err(|error| format!("Default example working directory create failed: {error}"))?;
    std::fs::copy(&default_path, &working_path)
        .map_err(|error| format!("Default example working copy failed: {error}"))?;
    Ok(working_path)
}

pub(crate) fn project_path_for_mutable_session(project_path: PathBuf) -> Result<PathBuf, String> {
    if is_tracked_synthetic_example_path(&project_path)? {
        return default_project_working_copy_path(&project_path);
    }

    Ok(project_path)
}

pub(crate) fn is_tracked_synthetic_example_path(project_path: &Path) -> Result<bool, String> {
    if project_path.file_name().and_then(|name| name.to_str())
        != Some("synthetic-studio-project.json")
    {
        return Ok(false);
    }

    if !project_path.is_file() {
        return Ok(false);
    }

    let project_path = normalize_verbatim_path(
        std::fs::canonicalize(project_path)
            .map_err(|error| format!("Project path resolve failed: {error}"))?,
    );
    Ok(project_path
        .parent()
        .and_then(|parent| parent.file_name())
        .and_then(|name| name.to_str())
        == Some("examples"))
}
