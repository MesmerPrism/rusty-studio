use super::super::*;

pub(crate) fn shell_release_candidate_review_selection_status_label(
    status: StudioShellReleaseCandidateReviewSelectionStatus,
) -> &'static str {
    match status {
        StudioShellReleaseCandidateReviewSelectionStatus::Selected => "selected",
        StudioShellReleaseCandidateReviewSelectionStatus::Missing => "missing",
        StudioShellReleaseCandidateReviewSelectionStatus::Empty => "empty",
    }
}

pub(crate) fn shell_release_candidate_review_status_label(
    status: StudioShellReleaseCandidateReviewStatus,
) -> &'static str {
    match status {
        StudioShellReleaseCandidateReviewStatus::Ready => "ready",
        StudioShellReleaseCandidateReviewStatus::Blocked => "blocked",
        StudioShellReleaseCandidateReviewStatus::Rejected => "rejected",
    }
}
