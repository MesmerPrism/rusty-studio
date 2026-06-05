use std::path::{Path, PathBuf};

use super::base::studio_shell_handoffs_dir;

pub(crate) fn shell_hostess_handoff_package_output_path(project_path: &Path) -> PathBuf {
    studio_shell_handoffs_dir(project_path).join("shell-hostess-handoff-package.json")
}

pub(crate) fn shell_hostess_owner_intake_output_path(project_path: &Path) -> PathBuf {
    studio_shell_handoffs_dir(project_path).join("shell-hostess-owner-intake.json")
}
