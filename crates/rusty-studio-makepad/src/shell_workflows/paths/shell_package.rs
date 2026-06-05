use std::path::{Path, PathBuf};

pub(crate) fn shell_export_package_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-export-package.json")
}

pub(crate) fn shell_export_package_baseline_manifest_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-export-package-baseline.json")
}

pub(crate) fn shell_export_package_baseline_archive_dir(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("export-package-baselines")
}

pub(crate) fn shell_export_package_baseline_archive_package_output_path(
    project_path: &Path,
    baseline_id: &str,
) -> PathBuf {
    shell_export_package_baseline_archive_dir(project_path)
        .join(format!("{baseline_id}.package.json"))
}

pub(crate) fn shell_export_package_baseline_archive_manifest_output_path(
    project_path: &Path,
    baseline_id: &str,
) -> PathBuf {
    shell_export_package_baseline_archive_dir(project_path)
        .join(format!("{baseline_id}.baseline.json"))
}

pub(crate) fn shell_export_package_baseline_index_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-export-package-baselines.json")
}
