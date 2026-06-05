use super::super::super::super::*;
use super::default_id::next_shell_hostess_staging_acceptance_default_id;
use super::ShellHostessStagingAcceptanceSelectionResult;

pub(crate) fn select_next_shell_hostess_staging_acceptance_default_for_project_source(
    project_path: &Path,
) -> ShellHostessStagingAcceptanceSelectionResult {
    let index_path = shell_hostess_staging_acceptance_index_output_path(project_path);
    let index = load_shell_hostess_staging_acceptance_index(&index_path)
        .map_err(|error| format!("Shell Hostess staging acceptance index load failed: {error}"))?;
    let acceptance_id = next_shell_hostess_staging_acceptance_default_id(&index)?;
    let archive_path = index
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
        load_shell_hostess_staging_acceptance_manifest(&archive_path).map_err(|error| {
            format!("Shell Hostess staging acceptance identity load failed: {error}")
        })?;
    let promoted =
        promote_shell_hostess_staging_acceptance_index_default(&index, &acceptance.acceptance_id)
            .ok_or_else(|| {
            format!(
                "Shell Hostess staging acceptance index does not contain acceptance {}",
                acceptance.acceptance_id
            )
        })?;
    save_json(&index_path, &promoted)
        .map_err(|error| format!("Shell Hostess staging acceptance index save failed: {error}"))?;
    let current_path = shell_hostess_staging_acceptance_manifest_output_path(project_path);
    save_json(&current_path, &acceptance).map_err(|error| {
        format!("Shell Hostess staging acceptance current identity save failed: {error}")
    })?;
    Ok((acceptance, promoted, current_path, index_path))
}
