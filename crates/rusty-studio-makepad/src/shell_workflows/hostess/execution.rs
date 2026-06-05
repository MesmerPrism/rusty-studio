use super::super::*;

pub(crate) fn shell_hostess_staging_execution_request_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellHostessStagingExecutionRequestReport, PathBuf), String> {
    let index_path = shell_hostess_staging_acceptance_index_output_path(project_path);
    let index = load_shell_hostess_staging_acceptance_index(&index_path)
        .map_err(|error| format!("Shell Hostess staging acceptance index load failed: {error}"))?;
    let Some(acceptance_index_entry) =
        select_shell_hostess_staging_acceptance_index_entry(&index, None)
    else {
        return Err(
            "Shell Hostess staging acceptance index does not contain a selected acceptance"
                .to_string(),
        );
    };
    let acceptance_path = acceptance_index_entry
        .acceptance_manifest_path
        .as_ref()
        .map(PathBuf::from)
        .ok_or_else(|| {
            "Selected acceptance index entry does not include an acceptance manifest path"
                .to_string()
        })?;
    let acceptance =
        load_shell_hostess_staging_acceptance_manifest(&acceptance_path).map_err(|error| {
            format!("Shell Hostess staging acceptance identity load failed: {error}")
        })?;
    let checklist_path = PathBuf::from(&acceptance.checklist_path);
    let checklist =
        load_shell_hostess_staging_acceptance_checklist(&checklist_path).map_err(|error| {
            format!("Shell Hostess staging acceptance checklist load failed: {error}")
        })?;
    let handoff_path = checklist
        .handoff_path
        .as_ref()
        .map(PathBuf::from)
        .ok_or_else(|| {
            "Selected acceptance checklist does not include a handoff path".to_string()
        })?;
    let handoff = load_shell_hostess_staging_handoff_envelope(&handoff_path)
        .map_err(|error| format!("Shell Hostess staging handoff load failed: {error}"))?;
    let report = shell_hostess_staging_execution_request_for_acceptance_index_entry(
        &index,
        Some(&index_path),
        acceptance_index_entry,
        Some(&acceptance_path),
        &acceptance,
        &checklist,
        Some(&handoff_path),
        &handoff,
    );
    let output_path = shell_hostess_staging_execution_request_output_path(project_path);
    save_json(&output_path, &report)
        .map_err(|error| format!("Shell Hostess staging execution request save failed: {error}"))?;
    Ok((report, output_path))
}
