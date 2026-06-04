use super::*;

pub(crate) fn shell_release_candidate_review_status(
    report: &StudioShellReleaseCandidateReviewReport,
    output_path: &Path,
) -> String {
    let status = shell_release_candidate_review_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let handoff_status = validation_status_label(report.handoff_status);
    let acceptance_selection = shell_handoff_acceptance_baseline_selection_status_label(
        report.acceptance_baseline_selection.status,
    );
    let acceptance_selected = report
        .acceptance_baseline_selection
        .selected_baseline_id
        .as_deref()
        .unwrap_or("none");
    let acceptance_comparison = report
        .acceptance_comparison
        .as_ref()
        .map(|comparison| shell_handoff_acceptance_comparison_status_label(comparison.status))
        .unwrap_or("missing");
    let export_package_selection = shell_export_package_baseline_selection_status_label(
        report.export_package_baseline_selection.status,
    );
    let export_package_selected = report
        .export_package_baseline_selection
        .selected_baseline_id
        .as_deref()
        .unwrap_or("none");
    let export_package_comparison = report
        .export_package_comparison
        .as_ref()
        .map(|comparison| shell_export_package_comparison_status_label(comparison.status))
        .unwrap_or("missing");
    let failed_checks = report
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();
    let first_issue = report
        .checks
        .iter()
        .find(|check| check.status == StudioValidationStatus::Fail)
        .and_then(|check| check.issue_code.as_deref())
        .unwrap_or("none");
    format!(
        "shell release candidate review {status}; issue {issue}\n  review: {}\n  manifest: {} rev {}; handoff {handoff_status}; ready {}; failed {}; missing bundles {}\n  acceptance baseline: {acceptance_selection}; selected {acceptance_selected}; comparison {acceptance_comparison}\n  export package baseline: {export_package_selection}; selected {export_package_selected}; comparison {export_package_comparison}\n  authority: command {}; host {}; studio {}; policy {}; owner {}\n  checks: {}; failed {}; first issue {}\n  prohibited: {}",
        output_path.display(),
        report.project_id,
        report.project_revision,
        report.handoff_ready_count,
        report.handoff_failed_count,
        report.handoff_missing_bundle_count,
        report.command_session_authority,
        report.install_launch_evidence_authority,
        report.studio_role,
        report.execution_policy,
        report.review_owner,
        report.checks.len(),
        failed_checks,
        first_issue,
        if report.prohibited_actions.is_empty() {
            "none".to_string()
        } else {
            report.prohibited_actions.join(", ")
        }
    )
}

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

pub(crate) fn shell_release_candidate_review_manifest_append_status(
    review: &StudioShellReleaseCandidateReviewReport,
    candidate: &StudioShellReleaseCandidateReviewManifest,
    index: &StudioShellReleaseCandidateReviewIndex,
    review_path: &Path,
    candidate_path: &Path,
    index_path: &Path,
) -> String {
    let selection = summarize_shell_release_candidate_review_index_selection(
        index,
        Some(index_path),
        Some(&candidate.candidate_id),
    );
    format!(
        "release candidate archived\n  candidate: {} ({})\n  identity: {}\n  review artifact: {}\n{}\n{}\n{}",
        candidate.candidate_id,
        candidate.label,
        candidate_path.display(),
        review_path.display(),
        shell_release_candidate_review_selection_status(&selection),
        shell_release_candidate_review_index_status(index, index_path),
        shell_release_candidate_review_status(review, review_path)
    )
}

pub(crate) fn shell_release_candidate_review_index_status(
    index: &StudioShellReleaseCandidateReviewIndex,
    index_path: &Path,
) -> String {
    let default = index.default_candidate_id.as_deref().unwrap_or("none");
    let projects = if index.project_ids.is_empty() {
        "none".to_string()
    } else {
        index.project_ids.join(", ")
    };
    let manifests = if index.manifest_ids.is_empty() {
        "none".to_string()
    } else {
        index.manifest_ids.join(", ")
    };
    let rows = index
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let status = shell_release_candidate_review_status_label(entry.status);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            let manifest_path = entry
                .candidate_manifest_path
                .as_deref()
                .unwrap_or("unknown");
            let acceptance = entry
                .acceptance_comparison_status
                .map(shell_handoff_acceptance_comparison_status_label)
                .unwrap_or("missing");
            let export_package = entry
                .export_package_comparison_status
                .map(shell_export_package_comparison_status_label)
                .unwrap_or("missing");
            format!(
                "{} [{}] project {} rev {}; handoff ready {}; failed {}; missing {}; acceptance {}; package {}; checks failed {}; manifest {}; issue {}",
                entry.candidate_id,
                status,
                entry.project_id,
                entry.project_revision,
                entry.handoff_ready_count,
                entry.handoff_failed_count,
                entry.handoff_missing_bundle_count,
                acceptance,
                export_package,
                entry.failed_check_count,
                manifest_path,
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "release candidate index slots {}; default {}; ready {}; blocked {}; rejected {}\n  index: {}\n  projects: {}\n  manifests: {}\n  entries:\n  {}",
        index.candidate_count,
        default,
        index.ready_candidate_count,
        index.blocked_candidate_count,
        index.rejected_candidate_count,
        index_path.display(),
        projects,
        manifests,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}

pub(crate) fn shell_release_candidate_review_selection_status(
    report: &StudioShellReleaseCandidateReviewSelectionReport,
) -> String {
    let status = shell_release_candidate_review_selection_status_label(report.status);
    let requested = report.requested_candidate_id.as_deref().unwrap_or("none");
    let default = report.default_candidate_id.as_deref().unwrap_or("none");
    let selected = report.selected_candidate_id.as_deref().unwrap_or("none");
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let index_path = report.index_path.as_deref().unwrap_or("not saved");
    let rows = report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let entry_status = shell_release_candidate_review_status_label(entry.status);
            let entry_issue = entry.issue_code.as_deref().unwrap_or("none");
            let manifest_path = entry
                .candidate_manifest_path
                .as_deref()
                .unwrap_or("unknown");
            let selected_flag = if entry.selected { "yes" } else { "no" };
            let default_flag = if entry.default { "yes" } else { "no" };
            let acceptance = entry
                .acceptance_comparison_status
                .map(shell_handoff_acceptance_comparison_status_label)
                .unwrap_or("missing");
            let export_package = entry
                .export_package_comparison_status
                .map(shell_export_package_comparison_status_label)
                .unwrap_or("missing");
            format!(
                "{} [{}] selected {}; default {}; acceptance {}; package {}; checks failed {}; manifest {}; issue {}",
                entry.candidate_id,
                entry_status,
                selected_flag,
                default_flag,
                acceptance,
                export_package,
                entry.failed_check_count,
                manifest_path,
                entry_issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "release candidate selection {status}; requested {requested}; default {default}; selected {selected}; slots {}; ready {}; blocked {}; rejected {}; issue {issue}\n  index: {}\n  entries:\n  {}",
        report.candidate_count,
        report.ready_candidate_count,
        report.blocked_candidate_count,
        report.rejected_candidate_count,
        index_path,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}

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
