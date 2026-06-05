use std::path::{Path, PathBuf};

pub(super) fn studio_shell_handoffs_dir(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
}
