use super::super::super::*;
use super::default_id::next_shell_release_candidate_default_id;
use super::ShellReleaseCandidateSelectionResult;

pub(crate) fn select_next_shell_release_candidate_default_for_project_source(
    project_path: &Path,
) -> ShellReleaseCandidateSelectionResult {
    let index_path = shell_release_candidate_review_index_output_path(project_path);
    let index = load_shell_release_candidate_review_index(&index_path)
        .map_err(|error| format!("Shell release candidate index load failed: {error}"))?;
    let candidate_id = next_shell_release_candidate_default_id(&index)?;
    let candidate_path = index
        .entries
        .iter()
        .find(|entry| entry.candidate_id == candidate_id)
        .and_then(|entry| entry.candidate_manifest_path.as_ref())
        .map(PathBuf::from)
        .ok_or_else(|| {
            format!(
                "Shell release candidate index entry {candidate_id} does not include a manifest path"
            )
        })?;
    let candidate = load_shell_release_candidate_review_manifest(&candidate_path)
        .map_err(|error| format!("Shell release candidate identity load failed: {error}"))?;
    let promoted =
        promote_shell_release_candidate_review_index_default(&index, &candidate.candidate_id)
            .ok_or_else(|| {
                format!(
                    "Shell release candidate index does not contain candidate {}",
                    candidate.candidate_id
                )
            })?;
    save_json(&index_path, &promoted)
        .map_err(|error| format!("Shell release candidate index save failed: {error}"))?;
    Ok((candidate, promoted, candidate_path, index_path))
}
