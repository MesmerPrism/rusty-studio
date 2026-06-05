use super::super::*;

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
