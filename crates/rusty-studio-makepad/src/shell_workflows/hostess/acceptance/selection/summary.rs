use super::super::super::super::*;
use super::ShellHostessStagingAcceptanceSelectionResult;

pub(crate) fn shell_hostess_staging_acceptance_summary_for_project_source(
    project_path: &Path,
) -> ShellHostessStagingAcceptanceSelectionResult {
    let index_path = shell_hostess_staging_acceptance_index_output_path(project_path);
    let index = load_shell_hostess_staging_acceptance_index(&index_path)
        .map_err(|error| format!("Shell Hostess staging acceptance index load failed: {error}"))?;
    let acceptance_id = index
        .default_acceptance_id
        .as_deref()
        .ok_or_else(|| "Shell Hostess staging acceptance index has no default entry".to_string())?;
    let acceptance_path = index
        .entries
        .iter()
        .find(|entry| entry.acceptance_id == acceptance_id)
        .and_then(|entry| entry.acceptance_manifest_path.as_ref())
        .map(PathBuf::from)
        .ok_or_else(|| {
            format!(
                "Shell Hostess staging acceptance index entry {acceptance_id} does not include a manifest path"
            )
        })?;
    let acceptance =
        load_shell_hostess_staging_acceptance_manifest(&acceptance_path).map_err(|error| {
            format!("Shell Hostess staging acceptance identity load failed: {error}")
        })?;
    Ok((acceptance, index, acceptance_path, index_path))
}
