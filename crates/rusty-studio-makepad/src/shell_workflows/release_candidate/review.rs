use super::super::*;

pub(crate) fn shell_release_candidate_review_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellReleaseCandidateReviewReport, PathBuf), String> {
    let manifest_path = shell_handoff_manifest_output_path(project_path);
    let manifest = load_shell_handoff_manifest(&manifest_path)
        .map_err(|error| format!("Shell handoff manifest load failed: {error}"))?;
    let acceptance_index_path = shell_handoff_acceptance_baseline_index_output_path(project_path);
    let acceptance_index = load_shell_handoff_acceptance_baseline_index(&acceptance_index_path)
        .map_err(|error| format!("Baseline acceptance index load failed: {error}"))?;
    let export_package_index_path = shell_export_package_baseline_index_output_path(project_path);
    let export_package_index = load_shell_export_package_baseline_index(&export_package_index_path)
        .map_err(|error| format!("Export package baseline index load failed: {error}"))?;
    let report = shell_release_candidate_review_for_manifest(
        &manifest,
        Some(&manifest_path),
        &acceptance_index,
        Some(&acceptance_index_path),
        None,
        &export_package_index,
        Some(&export_package_index_path),
        None,
    );
    let output_path = shell_release_candidate_review_output_path(project_path);
    save_json(&output_path, &report)
        .map_err(|error| format!("Shell release candidate review save failed: {error}"))?;
    Ok((report, output_path))
}
