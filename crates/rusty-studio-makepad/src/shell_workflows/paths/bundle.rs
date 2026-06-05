use std::path::{Path, PathBuf};

pub(crate) fn selected_shell_bundle_root_dir(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-selected-shell")
}

pub(crate) fn selected_shell_bundle_output_dir(project_path: &Path, graph_id: &str) -> PathBuf {
    selected_shell_bundle_root_dir(project_path).join(graph_id)
}
