use super::super::super::*;

pub(crate) fn shell_hostess_staging_acceptance_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellHostessStagingAcceptanceChecklistReport, PathBuf), String> {
    let handoff_path = shell_hostess_staging_handoff_output_path(project_path);
    let handoff = load_shell_hostess_staging_handoff_envelope(&handoff_path)
        .map_err(|error| format!("Shell Hostess staging handoff load failed: {error}"))?;
    let report =
        shell_hostess_staging_acceptance_checklist_for_handoff(&handoff, Some(&handoff_path));
    let output_path = shell_hostess_staging_acceptance_output_path(project_path);
    save_json(&output_path, &report).map_err(|error| {
        format!("Shell Hostess staging acceptance checklist save failed: {error}")
    })?;
    Ok((report, output_path))
}
