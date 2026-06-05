use std::path::{Path, PathBuf};

use super::base::studio_shell_handoffs_dir;

pub(crate) fn shell_hostess_staging_acceptance_output_path(project_path: &Path) -> PathBuf {
    studio_shell_handoffs_dir(project_path).join("shell-hostess-staging-acceptance-checklist.json")
}

pub(crate) fn shell_hostess_staging_acceptance_manifest_output_path(
    project_path: &Path,
) -> PathBuf {
    studio_shell_handoffs_dir(project_path).join("shell-hostess-staging-acceptance-manifest.json")
}

pub(crate) fn shell_hostess_staging_acceptance_archive_dir(project_path: &Path) -> PathBuf {
    studio_shell_handoffs_dir(project_path).join("hostess-staging-acceptances")
}

pub(crate) fn shell_hostess_staging_acceptance_archive_checklist_output_path(
    project_path: &Path,
    acceptance_id: &str,
) -> PathBuf {
    shell_hostess_staging_acceptance_archive_dir(project_path)
        .join(format!("{acceptance_id}.checklist.json"))
}

pub(crate) fn shell_hostess_staging_acceptance_archive_manifest_output_path(
    project_path: &Path,
    acceptance_id: &str,
) -> PathBuf {
    shell_hostess_staging_acceptance_archive_dir(project_path)
        .join(format!("{acceptance_id}.acceptance.json"))
}

pub(crate) fn shell_hostess_staging_acceptance_index_output_path(project_path: &Path) -> PathBuf {
    studio_shell_handoffs_dir(project_path).join("shell-hostess-staging-acceptances.json")
}

pub(crate) fn shell_hostess_staging_acceptance_comparison_output_path(
    project_path: &Path,
) -> PathBuf {
    studio_shell_handoffs_dir(project_path).join("shell-hostess-staging-acceptance-comparison.json")
}
