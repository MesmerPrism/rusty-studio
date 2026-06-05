use super::super::*;

pub(crate) fn shell_handoff_acceptance_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellHandoffAcceptanceChecklistReport, PathBuf), String> {
    let project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let bundle_root = selected_shell_bundle_root_dir(project_path);
    let report = shell_handoff_acceptance_checklist_for_project(
        &project,
        project_path.parent(),
        &bundle_root,
    );
    Ok((report, bundle_root))
}
