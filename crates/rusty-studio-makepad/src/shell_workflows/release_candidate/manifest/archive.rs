use super::super::super::*;
use super::super::review::*;
use super::identity::next_shell_release_candidate_archive_identity;
use super::ShellReleaseCandidateReviewManifestWriteResult;

pub(crate) fn append_shell_release_candidate_review_manifest_for_project_source(
    project_path: &Path,
) -> ShellReleaseCandidateReviewManifestWriteResult {
    let (review, _) = shell_release_candidate_review_for_project_source(project_path)?;
    let index_path = shell_release_candidate_review_index_output_path(project_path);
    let existing_index = if index_path.is_file() {
        Some(
            load_shell_release_candidate_review_index(&index_path)
                .map_err(|error| format!("Shell release candidate index load failed: {error}"))?,
        )
    } else {
        None
    };
    let (candidate_id, label) =
        next_shell_release_candidate_archive_identity(&review, existing_index.as_ref());
    let review_path =
        shell_release_candidate_review_archive_report_output_path(project_path, &candidate_id);
    save_json(&review_path, &review)
        .map_err(|error| format!("Shell release candidate review archive save failed: {error}"))?;
    let candidate = shell_release_candidate_review_manifest_for_report(
        &review,
        &review_path,
        Some(&candidate_id),
        Some(&label),
    );
    let candidate_path =
        shell_release_candidate_review_archive_manifest_output_path(project_path, &candidate_id);
    save_json(&candidate_path, &candidate)
        .map_err(|error| format!("Shell release candidate identity save failed: {error}"))?;
    let index = if let Some(index) = existing_index.as_ref() {
        append_shell_release_candidate_review_index_manifests(
            index,
            vec![(candidate.clone(), Some(candidate_path.clone()))],
            Some(&candidate.candidate_id),
        )
    } else {
        shell_release_candidate_review_index_for_manifests(
            vec![(candidate.clone(), Some(candidate_path.clone()))],
            Some(&candidate.candidate_id),
        )
    };
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
