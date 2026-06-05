use std::path::{Path, PathBuf};

use super::base::studio_shell_handoffs_dir;

pub(crate) fn shell_hostess_staging_execution_request_output_path(project_path: &Path) -> PathBuf {
    studio_shell_handoffs_dir(project_path).join("shell-hostess-staging-execution-request.json")
}
