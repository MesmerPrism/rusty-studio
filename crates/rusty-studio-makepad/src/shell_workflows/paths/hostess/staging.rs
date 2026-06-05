use std::path::{Path, PathBuf};

use super::base::studio_shell_handoffs_dir;

pub(crate) fn shell_hostess_staging_preview_output_path(project_path: &Path) -> PathBuf {
    studio_shell_handoffs_dir(project_path).join("shell-hostess-staging-preview.json")
}

pub(crate) fn shell_hostess_staging_file_plan_output_path(project_path: &Path) -> PathBuf {
    studio_shell_handoffs_dir(project_path).join("shell-hostess-staging-file-plan.json")
}

pub(crate) fn shell_hostess_staging_handoff_output_path(project_path: &Path) -> PathBuf {
    studio_shell_handoffs_dir(project_path).join("shell-hostess-staging-handoff.json")
}
