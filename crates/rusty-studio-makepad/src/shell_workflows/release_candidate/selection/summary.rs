use super::super::super::*;
use super::ShellReleaseCandidateSelectionResult;

pub(crate) fn shell_release_candidate_review_manifest_summary_for_project_source(
    project_path: &Path,
) -> ShellReleaseCandidateSelectionResult {
    let candidate_path = shell_release_candidate_review_manifest_output_path(project_path);
    let candidate = load_shell_release_candidate_review_manifest(&candidate_path)
        .map_err(|error| format!("Shell release candidate identity load failed: {error}"))?;
    let index_path = shell_release_candidate_review_index_output_path(project_path);
    let index = load_shell_release_candidate_review_index(&index_path)
        .map_err(|error| format!("Shell release candidate index load failed: {error}"))?;
    Ok((candidate, index, candidate_path, index_path))
}
