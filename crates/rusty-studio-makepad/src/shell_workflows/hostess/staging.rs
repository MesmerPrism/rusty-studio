use super::super::*;

pub(crate) fn shell_hostess_staging_preview_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellHostessStagingPreviewManifest, PathBuf), String> {
    let intake_path = shell_hostess_owner_intake_output_path(project_path);
    let intake = load_shell_hostess_owner_intake_report(&intake_path)
        .map_err(|error| format!("Shell Hostess owner intake load failed: {error}"))?;
    let report = shell_hostess_staging_preview_for_owner_intake(&intake, Some(&intake_path));
    let output_path = shell_hostess_staging_preview_output_path(project_path);
    save_json(&output_path, &report)
        .map_err(|error| format!("Shell Hostess staging preview save failed: {error}"))?;
    Ok((report, output_path))
}

pub(crate) fn shell_hostess_staging_file_plan_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellHostessStagingFilePlan, PathBuf), String> {
    let preview_path = shell_hostess_staging_preview_output_path(project_path);
    let preview = load_shell_hostess_staging_preview_manifest(&preview_path)
        .map_err(|error| format!("Shell Hostess staging preview load failed: {error}"))?;
    let report = shell_hostess_staging_file_plan_for_preview(&preview, Some(&preview_path));
    let output_path = shell_hostess_staging_file_plan_output_path(project_path);
    save_json(&output_path, &report)
        .map_err(|error| format!("Shell Hostess staging file plan save failed: {error}"))?;
    Ok((report, output_path))
}

pub(crate) fn shell_hostess_staging_handoff_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellHostessStagingHandoffEnvelope, PathBuf), String> {
    let file_plan_path = shell_hostess_staging_file_plan_output_path(project_path);
    let file_plan = load_shell_hostess_staging_file_plan(&file_plan_path)
        .map_err(|error| format!("Shell Hostess staging file plan load failed: {error}"))?;
    let report =
        shell_hostess_staging_handoff_envelope_for_file_plan(&file_plan, Some(&file_plan_path));
    let output_path = shell_hostess_staging_handoff_output_path(project_path);
    save_json(&output_path, &report)
        .map_err(|error| format!("Shell Hostess staging handoff save failed: {error}"))?;
    Ok((report, output_path))
}
