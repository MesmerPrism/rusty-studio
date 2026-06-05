use super::super::super::*;

pub(crate) fn next_shell_release_candidate_archive_identity(
    review: &StudioShellReleaseCandidateReviewReport,
    index: Option<&StudioShellReleaseCandidateReviewIndex>,
) -> (String, String) {
    let status = shell_release_candidate_review_status_label(review.status);
    let base_id = format!(
        "{}.rev{}.{}",
        review.project_id, review.project_revision, status
    );
    let next_slot = index
        .map(|index| {
            index
                .entries
                .iter()
                .filter(|entry| {
                    entry.candidate_id == base_id
                        || entry
                            .candidate_id
                            .strip_prefix(base_id.as_str())
                            .is_some_and(|suffix| suffix.starts_with(".archive"))
                })
                .count()
                + 1
        })
        .unwrap_or(1);
    let candidate_id = if next_slot == 1 {
        base_id
    } else {
        format!("{base_id}.archive{next_slot}")
    };
    let label = if next_slot == 1 {
        format!(
            "{} revision {} {} release candidate",
            review.project_id, review.project_revision, status
        )
    } else {
        format!(
            "{} revision {} {} release candidate archive {}",
            review.project_id, review.project_revision, status, next_slot
        )
    };
    (candidate_id, label)
}
