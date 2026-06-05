use super::super::super::*;
use super::super::index::*;

pub(crate) fn shell_release_candidate_review_manifest_promote_status(
    candidate: &StudioShellReleaseCandidateReviewManifest,
    index: &StudioShellReleaseCandidateReviewIndex,
    candidate_path: &Path,
    index_path: &Path,
) -> String {
    let selection = summarize_shell_release_candidate_review_index_selection(
        index,
        Some(index_path),
        Some(&candidate.candidate_id),
    );
    format!(
        "release candidate default promoted\n  candidate: {} ({})\n  identity: {}\n{}\n{}",
        candidate.candidate_id,
        candidate.label,
        candidate_path.display(),
        shell_release_candidate_review_selection_status(&selection),
        shell_release_candidate_review_index_status(index, index_path)
    )
}

pub(crate) fn shell_release_candidate_review_manifest_select_status(
    candidate: &StudioShellReleaseCandidateReviewManifest,
    index: &StudioShellReleaseCandidateReviewIndex,
    candidate_path: &Path,
    index_path: &Path,
) -> String {
    let selection = summarize_shell_release_candidate_review_index_selection(
        index,
        Some(index_path),
        Some(&candidate.candidate_id),
    );
    format!(
        "release candidate default selected\n  candidate: {} ({})\n  identity: {}\n{}\n{}",
        candidate.candidate_id,
        candidate.label,
        candidate_path.display(),
        shell_release_candidate_review_selection_status(&selection),
        shell_release_candidate_review_index_status(index, index_path)
    )
}
