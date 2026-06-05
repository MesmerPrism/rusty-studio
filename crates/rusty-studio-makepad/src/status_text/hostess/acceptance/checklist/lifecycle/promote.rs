use super::super::super::super::super::*;
use super::super::super::index::*;

pub(crate) fn shell_hostess_staging_acceptance_promote_status(
    acceptance: &StudioShellHostessStagingAcceptanceManifest,
    index: &StudioShellHostessStagingAcceptanceIndex,
    acceptance_path: &Path,
    index_path: &Path,
) -> String {
    let selection = summarize_shell_hostess_staging_acceptance_index_selection(
        index,
        Some(index_path),
        Some(&acceptance.acceptance_id),
    );
    format!(
        "Hostess staging acceptance default promoted\n  acceptance: {} ({})\n  identity: {}\n{}\n{}",
        acceptance.acceptance_id,
        acceptance.label,
        acceptance_path.display(),
        shell_hostess_staging_acceptance_selection_status(&selection),
        shell_hostess_staging_acceptance_index_status(index, index_path)
    )
}
