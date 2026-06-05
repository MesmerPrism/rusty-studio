use super::super::super::*;
use super::super::review::*;
use super::ShellReleaseCandidateReviewManifestWriteResult;

pub(crate) fn write_shell_release_candidate_review_manifest_for_project_source(
    project_path: &Path,
) -> ShellReleaseCandidateReviewManifestWriteResult {
    let (review, review_path) = shell_release_candidate_review_for_project_source(project_path)?;
    let candidate =
        shell_release_candidate_review_manifest_for_report(&review, &review_path, None, None);
    let candidate_path = shell_release_candidate_review_manifest_output_path(project_path);
    save_json(&candidate_path, &candidate)
        .map_err(|error| format!("Shell release candidate identity save failed: {error}"))?;
    let index = shell_release_candidate_review_index_for_manifests(
        vec![(candidate.clone(), Some(candidate_path.clone()))],
        Some(&candidate.candidate_id),
    );
    let index_path = shell_release_candidate_review_index_output_path(project_path);
    save_json(&index_path, &index)
        .map_err(|error| format!("Shell release candidate index save failed: {error}"))?;
    Ok((
        review,
        candidate,
        index,
        review_path,
        candidate_path,
        index_path,
    ))
}
