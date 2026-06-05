use super::super::super::*;

mod actions;

use actions::shell_hostess_handoff_package_action_rows;

pub(crate) fn shell_hostess_handoff_package_status(
    report: &StudioShellHostessHandoffPackageReport,
    output_path: &Path,
) -> String {
    let status = shell_hostess_handoff_package_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let selected = report.selected_candidate_id.as_deref().unwrap_or("none");
    let candidate_path = report
        .candidate_manifest_path
        .as_deref()
        .unwrap_or("unknown");
    let review_path = report.review_path.as_deref().unwrap_or("unknown");
    let handoff_path = report.handoff_manifest_path.as_deref().unwrap_or("unknown");
    let acceptance = report
        .acceptance_comparison_status
        .map(shell_handoff_acceptance_comparison_status_label)
        .unwrap_or("missing");
    let export_package = report
        .export_package_comparison_status
        .map(shell_export_package_comparison_status_label)
        .unwrap_or("missing");
    let actions = shell_hostess_handoff_package_action_rows(report);
    let prohibited = if report.prohibited_actions.is_empty() {
        "none".to_string()
    } else {
        report.prohibited_actions.join(", ")
    };
    let failed_checks = report
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();
    format!(
        "shell Hostess handoff package {status}; selected {selected}; issue {issue}\n  package: {}\n  candidate: {}\n  review: {}\n  handoff manifest: {}\n  project: {} rev {}\n  handoff ready {}; failed {}; missing {}; acceptance {}; export package {}\n  authority: command {}; host {}; studio {}; policy {}; owner {}\n  actions:\n  {}\n  prohibited: {}\n  checks: {}; failed {}",
        output_path.display(),
        candidate_path,
        review_path,
        handoff_path,
        report.project_id.as_deref().unwrap_or("unknown"),
        report
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        report.handoff_ready_count,
        report.handoff_failed_count,
        report.handoff_missing_bundle_count,
        acceptance,
        export_package,
        report
            .command_session_authority
            .as_deref()
            .unwrap_or("unknown"),
        report
            .install_launch_evidence_authority
            .as_deref()
            .unwrap_or("unknown"),
        report.studio_role.as_deref().unwrap_or("unknown"),
        report.execution_policy,
        report.handoff_owner,
        if actions.is_empty() {
            "none".to_string()
        } else {
            actions
        },
        prohibited,
        report.checks.len(),
        failed_checks
    )
}
