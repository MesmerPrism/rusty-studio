use super::super::super::super::*;
use super::ShellHostessStagingAcceptanceSelectionResult;

pub(crate) fn promote_shell_hostess_staging_acceptance_default_for_project_source(
    project_path: &Path,
) -> ShellHostessStagingAcceptanceSelectionResult {
    let acceptance_path = shell_hostess_staging_acceptance_manifest_output_path(project_path);
    let acceptance =
        load_shell_hostess_staging_acceptance_manifest(&acceptance_path).map_err(|error| {
            format!("Shell Hostess staging acceptance identity load failed: {error}")
        })?;
    let index_path = shell_hostess_staging_acceptance_index_output_path(project_path);
    let index = load_shell_hostess_staging_acceptance_index(&index_path)
        .map_err(|error| format!("Shell Hostess staging acceptance index load failed: {error}"))?;
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
    Ok((acceptance, promoted, acceptance_path, index_path))
}
