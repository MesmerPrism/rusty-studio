use std::path::{Path, PathBuf};

pub(crate) fn shell_release_candidate_review_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-release-candidate-review.json")
}

pub(crate) fn shell_release_candidate_review_manifest_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-release-candidate-review-manifest.json")
}

pub(crate) fn shell_release_candidate_review_archive_dir(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("release-candidates")
}

pub(crate) fn shell_release_candidate_review_archive_report_output_path(
    project_path: &Path,
    candidate_id: &str,
) -> PathBuf {
    shell_release_candidate_review_archive_dir(project_path)
        .join(format!("{candidate_id}.review.json"))
}

pub(crate) fn shell_release_candidate_review_archive_manifest_output_path(
    project_path: &Path,
    candidate_id: &str,
) -> PathBuf {
    shell_release_candidate_review_archive_dir(project_path)
        .join(format!("{candidate_id}.candidate.json"))
}

pub(crate) fn shell_release_candidate_review_index_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-release-candidate-reviews.json")
}
