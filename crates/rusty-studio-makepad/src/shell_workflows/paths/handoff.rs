use std::path::{Path, PathBuf};

pub(crate) fn shell_handoff_manifest_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-handoffs.json")
}

pub(crate) fn shell_handoff_acceptance_checklist_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-handoff-acceptance-checklist.json")
}

pub(crate) fn shell_handoff_acceptance_baseline_manifest_output_path(
    project_path: &Path,
) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-handoff-acceptance-baseline.json")
}

pub(crate) fn shell_handoff_acceptance_baseline_archive_dir(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("baselines")
}

pub(crate) fn shell_handoff_acceptance_baseline_archive_checklist_output_path(
    project_path: &Path,
    baseline_id: &str,
) -> PathBuf {
    shell_handoff_acceptance_baseline_archive_dir(project_path)
        .join(format!("{baseline_id}.checklist.json"))
}

pub(crate) fn shell_handoff_acceptance_baseline_archive_manifest_output_path(
    project_path: &Path,
    baseline_id: &str,
) -> PathBuf {
    shell_handoff_acceptance_baseline_archive_dir(project_path)
        .join(format!("{baseline_id}.baseline.json"))
}

pub(crate) fn shell_handoff_acceptance_baseline_index_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-handoff-acceptance-baselines.json")
}
