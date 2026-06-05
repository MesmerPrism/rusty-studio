use super::super::super::*;
use super::super::index::*;
use super::super::review::*;

pub(crate) fn shell_release_candidate_review_manifest_status(
    review: &StudioShellReleaseCandidateReviewReport,
    candidate: &StudioShellReleaseCandidateReviewManifest,
    index: &StudioShellReleaseCandidateReviewIndex,
    review_path: &Path,
    candidate_path: &Path,
    index_path: &Path,
) -> String {
    let selection =
        summarize_shell_release_candidate_review_index_selection(index, Some(index_path), None);
    format!(
        "release candidate written\n  candidate: {} ({})\n  identity: {}\n  review artifact: {}\n{}\n{}\n{}",
        candidate.candidate_id,
        candidate.label,
        candidate_path.display(),
        review_path.display(),
        shell_release_candidate_review_selection_status(&selection),
        shell_release_candidate_review_index_status(index, index_path),
        shell_release_candidate_review_status(review, review_path)
    )
}
