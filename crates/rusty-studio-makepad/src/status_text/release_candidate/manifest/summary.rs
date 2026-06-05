use super::super::super::*;
use super::super::index::*;

pub(crate) fn shell_release_candidate_review_manifest_summary_status(
    candidate: &StudioShellReleaseCandidateReviewManifest,
    index: &StudioShellReleaseCandidateReviewIndex,
    candidate_path: &Path,
    index_path: &Path,
) -> String {
    let selection =
        summarize_shell_release_candidate_review_index_selection(index, Some(index_path), None);
    let status = shell_release_candidate_review_status_label(candidate.status);
    let issue = candidate.issue_code.as_deref().unwrap_or("none");
    let acceptance = candidate
        .acceptance_comparison_status
        .map(shell_handoff_acceptance_comparison_status_label)
        .unwrap_or("missing");
    let export_package = candidate
        .export_package_comparison_status
        .map(shell_export_package_comparison_status_label)
        .unwrap_or("missing");
    format!(
        "release candidate summary {status}; candidate {} ({}); project {} rev {}; manifest {}; issue {issue}\n  identity: {}\n  review artifact: {}\n  handoff ready {}; failed {}; missing {}; acceptance {}; export package {}; checks {}; failed {}\n  authority: command {}; host {}; studio {}; policy {}; owner {}\n{}\n{}",
        candidate.candidate_id,
        candidate.label,
        candidate.project_id,
        candidate.project_revision,
        candidate.manifest_id,
        candidate_path.display(),
        candidate.review_path,
        candidate.handoff_ready_count,
        candidate.handoff_failed_count,
        candidate.handoff_missing_bundle_count,
        acceptance,
        export_package,
        candidate.check_count,
        candidate.failed_check_count,
        candidate.command_session_authority,
        candidate.install_launch_evidence_authority,
        candidate.studio_role,
        candidate.execution_policy,
        candidate.review_owner,
        shell_release_candidate_review_selection_status(&selection),
        shell_release_candidate_review_index_status(index, index_path)
    )
}
