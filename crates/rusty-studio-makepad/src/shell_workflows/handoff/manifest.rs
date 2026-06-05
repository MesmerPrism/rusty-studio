use super::super::*;

pub(crate) fn write_shell_handoff_manifest_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellHandoffManifest, PathBuf), String> {
    let project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let bundle_root = selected_shell_bundle_root_dir(project_path);
    let manifest =
        shell_handoff_manifest_for_project(&project, project_path.parent(), &bundle_root);
    let output_path = shell_handoff_manifest_output_path(project_path);
    save_json(&output_path, &manifest)
        .map_err(|error| format!("Shell handoff manifest save failed: {error}"))?;
    Ok((manifest, output_path))
}
