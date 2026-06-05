use super::super::*;

pub(crate) fn shell_hostess_handoff_package_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellHostessHandoffPackageReport, PathBuf), String> {
    let index_path = shell_release_candidate_review_index_output_path(project_path);
    let index = load_shell_release_candidate_review_index(&index_path)
        .map_err(|error| format!("Shell release candidate index load failed: {error}"))?;
    let report =
        shell_hostess_handoff_package_for_release_candidate_index(&index, Some(&index_path), None);
    let output_path = shell_hostess_handoff_package_output_path(project_path);
    save_json(&output_path, &report)
        .map_err(|error| format!("Shell Hostess handoff package save failed: {error}"))?;
    Ok((report, output_path))
}

pub(crate) fn shell_hostess_owner_intake_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellHostessOwnerIntakeReport, PathBuf), String> {
    let package_path = shell_hostess_handoff_package_output_path(project_path);
    let package = load_shell_hostess_handoff_package_report(&package_path)
        .map_err(|error| format!("Shell Hostess handoff package load failed: {error}"))?;
    let report = shell_hostess_owner_intake_for_handoff_package(&package, Some(&package_path));
    let output_path = shell_hostess_owner_intake_output_path(project_path);
    save_json(&output_path, &report)
        .map_err(|error| format!("Shell Hostess owner intake save failed: {error}"))?;
    Ok((report, output_path))
}
